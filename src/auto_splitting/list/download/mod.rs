use reqwest::Client;

/// An asynchronous HTTP client for downloading the auto splitter list and its
/// referenced files into memory. Caching and persistence are left to the caller.
/// On native platforms, downloading requires a Tokio runtime.
pub struct Downloader {
    client: Client,
}

impl Downloader {
    /// Creates a downloader. This can fail if the HTTP client cannot initialize.
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Client::builder().build()?,
        })
    }

    /// Uses an existing HTTP client, allowing the caller to configure timeouts,
    /// proxies, and other request options or share an existing connection pool.
    pub const fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Downloads an XML list from the given URL into an owned string. Pass
    /// [`super::DEFAULT_LIST_URL`] to download the community-maintained list.
    /// Borrow the text with [`super::List::new`] to look up games on demand.
    /// This only downloads the text; parsing errors are reported by queries.
    pub async fn download_list(&self, url: &str) -> Result<String, reqwest::Error> {
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    /// Downloads a file referenced by [`super::AutoSplitter::urls`] into memory.
    /// The caller decides which URLs to download and how to load or persist the
    /// returned bytes. HTTP errors are returned without writing any files.
    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>, reqwest::Error> {
        let bytes = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        Ok(bytes.to_vec())
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests;
