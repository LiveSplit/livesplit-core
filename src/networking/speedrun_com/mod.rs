//! Speedrun.com API integration.
//!
//! Enabled by the `speedrun-com` Cargo feature.

mod models;
mod transport;

pub use models::{
    Artwork, Asset, Assets, Category, CategoryKind, Embedded, Game, GameHeader, Names, Platform,
    Region, Rules, TimingMethod, Variable, VariableScope, VariableValue, VariableValues,
};

use reqwest::{Client as HttpClient, Url};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use std::{collections::HashSet, error, fmt, time::Duration};

/// The production v1 metadata endpoint.
const BASE_URL: &str = "https://www.speedrun.com/api/v1/";
// Bulk mode allows 1000 compact games per page; ordinary collections allow 200.
const BULK_PAGE_SIZE: &str = "1000";
const PAGE_SIZE: &str = "200";
// Apply the deadline per page so a large catalogue can keep making progress
// while stalled requests fail promptly.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
// Game icons/covers are small. Bound downloaded bytes even if a server sends an
// incorrect or absent Content-Length, before handing the image to a decoder.
const MAX_ARTWORK_BYTES: usize = 8 * 1024 * 1024;

/// An error from the Speedrun.com client.
#[derive(Debug)]
pub enum Error {
    /// HTTP, status-code or response deserialization failure.
    Http(reqwest::Error),
    /// A supplied URL was malformed, insecure or outside the expected API route.
    InvalidUrl,
    /// Pagination repeated an already visited page.
    PaginationCycle,
    /// The game has no artwork of the requested kind.
    MissingArtwork,
    /// An artwork response exceeded the documented byte limit.
    ArtworkTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(error) => error.fmt(f),
            Self::InvalidUrl => f.write_str("invalid Speedrun.com URL"),
            Self::PaginationCycle => f.write_str("Speedrun.com pagination repeated a page"),
            Self::MissingArtwork => f.write_str("the game has no requested artwork"),
            Self::ArtworkTooLarge => f.write_str("Speedrun.com artwork exceeds the byte limit"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Http(error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

#[derive(Default, Deserialize)]
struct Pagination {
    #[serde(default)]
    links: Option<Vec<Link>>,
}

#[derive(Deserialize)]
struct Link {
    rel: String,
    uri: String,
}

#[derive(Deserialize)]
struct Paginated<T> {
    data: Vec<T>,
    pagination: Pagination,
}

/// One complete page. The opaque continuation can only be fetched by this client.
#[derive(Debug)]
pub struct Page<T> {
    /// The items on this page, in the site's order.
    pub items: Vec<T>,
    next: Option<Url>,
}

/// A reusable, pooled HTTP client. Clones share connections.
#[derive(Clone)]
pub struct Client {
    http: HttpClient,
}

impl Client {
    /// Constructs a client with the application's User-Agent, e.g. `Timer/1.0`.
    ///
    /// Invalid header values return [`Error::Http`]. Browser Fetch may override
    /// the User-Agent header.
    ///
    /// Native requests require a Tokio 1.x runtime with I/O and time drivers
    /// enabled. Dropping a request future cancels that request.
    pub fn new(user_agent: &str) -> Result<Self, Error> {
        Ok(Self {
            http: transport::client(user_agent)?,
        })
    }

    fn url(segments: &[&str]) -> Result<Url, Error> {
        // The URL parser normalizes dot segments and strips some control
        // characters. Reject them before encoding, including disguised dots such
        // as ".\t.", so an ID cannot silently select a different game or route.
        if segments.iter().any(|segment| {
            matches!(*segment, "" | "." | "..") || segment.chars().any(char::is_control)
        }) {
            return Err(Error::InvalidUrl);
        }

        let mut url = Url::parse(BASE_URL).expect("the constant API base URL is valid");
        url.path_segments_mut()
            .expect("the HTTPS base URL supports path segments")
            .pop_if_empty()
            .extend(segments);

        Ok(url)
    }

    async fn request<T: DeserializeOwned>(&self, url: Url) -> Result<T, Error> {
        Ok(self
            .http
            .get(url)
            .timeout(REQUEST_TIMEOUT)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    async fn page<T: DeserializeOwned>(&self, url: Url) -> Result<Page<T>, Error> {
        let response: Paginated<T> = self.request(url.clone()).await?;
        let next = response
            .pagination
            .links
            .iter()
            .flatten()
            .find(|link| link.rel == "next")
            .map(|link| continuation(&url, &link.uri))
            .transpose()?;

        Ok(Page {
            items: response.data,
            next,
        })
    }

    /// Lists compact game headers using the same bulk mode as the web editor.
    pub async fn game_headers(&self) -> Result<Page<GameHeader>, Error> {
        let mut url = Self::url(&["games"])?;
        url.query_pairs_mut()
            .extend_pairs([("_bulk", "yes"), ("max", BULK_PAGE_SIZE)]);

        self.page(url).await
    }

    /// Searches the site for games. The query is encoded as a single parameter.
    pub async fn games(&self, name: &str) -> Result<Page<GameHeader>, Error> {
        let mut url = Self::url(&["games"])?;
        url.query_pairs_mut().append_pair("name", name);

        self.page(url).await
    }

    /// Gets a game by stable ID or abbreviation, embedding its metadata variables.
    /// Empty IDs, dot path segments (`.` or `..`) and IDs containing control
    /// characters return [`Error::InvalidUrl`].
    pub async fn game(&self, id: &str) -> Result<Game, Error> {
        let mut url = Self::url(&["games", id])?;
        url.query_pairs_mut().append_pair("embed", "variables");

        Ok(self.request::<Response<Game>>(url).await?.data)
    }

    /// Gets every category, including individual-level categories, in the site's
    /// order. Editors can filter by [`CategoryKind::PerGame`].
    /// Empty game IDs, dot path segments and IDs containing control characters
    /// return [`Error::InvalidUrl`].
    pub async fn categories(&self, game: &str) -> Result<Vec<Category>, Error> {
        Ok(self
            .request::<Response<Vec<Category>>>(Self::url(&["games", game, "categories"])?)
            .await?
            .data)
    }

    /// Lists hardware platforms, following pagination through [`Self::collect`].
    pub async fn platforms(&self) -> Result<Page<Platform>, Error> {
        let mut url = Self::url(&["platforms"])?;
        url.query_pairs_mut().append_pair("max", PAGE_SIZE);

        self.page(url).await
    }

    /// Lists release regions, following pagination through [`Self::collect`].
    pub async fn regions(&self) -> Result<Page<Region>, Error> {
        let mut url = Self::url(&["regions"])?;
        url.query_pairs_mut().append_pair("max", PAGE_SIZE);

        self.page(url).await
    }

    /// Fetches the next page. The current page remains available for retries.
    pub async fn next<T: DeserializeOwned>(
        &self,
        page: &Page<T>,
    ) -> Result<Option<Page<T>>, Error> {
        match &page.next {
            Some(url) => self.page(url.clone()).await.map(Some),
            None => Ok(None),
        }
    }

    /// Collects every page, propagating failures from any page. Repeated
    /// continuation URLs return [`Error::PaginationCycle`].
    pub async fn collect<T: DeserializeOwned>(&self, mut page: Page<T>) -> Result<Vec<T>, Error> {
        let mut items = Vec::new();
        let mut visited = HashSet::new();

        loop {
            items.append(&mut page.items);
            let Some(url) = page.next else {
                return Ok(items);
            };

            if !visited.insert(url.clone()) {
                return Err(Error::PaginationCycle);
            }

            page = self.page(url).await?;
        }
    }

    /// Downloads real game artwork over HTTPS, rejecting site placeholders and
    /// responses larger than 8 MiB.
    pub async fn artwork(&self, game: &Game, kind: Artwork) -> Result<Vec<u8>, Error> {
        let asset = game.artwork(kind).ok_or(Error::MissingArtwork)?;

        // FIXME: Desktop icon and box-art downloads currently fail because
        // Speedrun.com asset endpoints return HTTP 403 HTML responses with
        // `cf-mitigated: challenge`. This is a Cloudflare browser challenge,
        // which the native HTTP client cannot complete; a browser User-Agent
        // alone does not satisfy it. Restoring downloads needs an asset
        // endpoint or access policy that permits native clients.
        self.image(&asset.uri).await
    }

    /// Downloads an HTTPS image as encoded bytes with the same timeout and
    /// 8 MiB limit as artwork.
    pub async fn image(&self, uri: &str) -> Result<Vec<u8>, Error> {
        let url = image_url(uri)?;
        let response = self
            .http
            .get(url)
            .timeout(REQUEST_TIMEOUT)
            .send()
            .await?
            .error_for_status()?;

        if response
            .content_length()
            .is_some_and(|len| len > MAX_ARTWORK_BYTES as u64)
        {
            return Err(Error::ArtworkTooLarge);
        }

        transport::read_artwork(response).await
    }
}

fn image_url(uri: &str) -> Result<Url, Error> {
    let url = Url::parse(uri).map_err(|_| Error::InvalidUrl)?;

    if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
        return Err(Error::InvalidUrl);
    }

    Ok(url)
}

fn continuation(current: &Url, uri: &str) -> Result<Url, Error> {
    let mut next = current.join(uri).map_err(|_| Error::InvalidUrl)?;

    // v1 still returns HTTP links. Upgrade them like the web implementation
    // and require continuations to stay on the current collection's origin and
    // path.
    if next.scheme() == "http" {
        next.set_scheme("https").map_err(|_| Error::InvalidUrl)?;
    }

    if next.origin() != current.origin()
        || next.path() != current.path()
        || !next.username().is_empty()
        || next.password().is_some()
        || next.fragment().is_some()
    {
        return Err(Error::InvalidUrl);
    }

    if next == *current {
        return Err(Error::PaginationCycle);
    }

    Ok(next)
}

#[cfg(test)]
mod tests;
