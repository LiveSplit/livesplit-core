//! Browser Fetch transport and buffered image responses.

use crate::networking::speedrun_com::{Error, MAX_ARTWORK_BYTES};

pub fn client(user_agent: &str) -> Result<reqwest::Client, Error> {
    Ok(reqwest::Client::builder().user_agent(user_agent).build()?)
}

pub async fn read_artwork(response: reqwest::Response) -> Result<Vec<u8>, Error> {
    let bytes = response.bytes().await?;

    if bytes.len() > MAX_ARTWORK_BYTES {
        return Err(Error::ArtworkTooLarge);
    }

    Ok(bytes.to_vec())
}
