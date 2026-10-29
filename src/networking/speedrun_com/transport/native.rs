//! Native HTTPS transport and streamed image responses.

use crate::networking::speedrun_com::{Error, MAX_ARTWORK_BYTES};

pub fn client(user_agent: &str) -> Result<reqwest::Client, Error> {
    Ok(builder(user_agent).build()?)
}

fn builder(user_agent: &str) -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .https_only(true)
        .user_agent(user_agent)
}

pub async fn read_artwork(mut response: reqwest::Response) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        if chunk.len() > MAX_ARTWORK_BYTES - bytes.len() {
            return Err(Error::ArtworkTooLarge);
        }

        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    //! A loopback HTTP fixture for inspecting application request headers.

    use super::builder;
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };

    #[tokio::test]
    #[ignore = "requires loopback TCP sockets"]
    async fn native_requests_identify_the_application() {
        // Bound the HTTP exchange. Each iteration reserves its own loopback port.
        const TIMEOUT: Duration = Duration::from_secs(5);

        for user_agent in [
            "ExampleTimer/1.2.3",
            "AnotherTimer/4.5 (+https://example.org)",
        ] {
            let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
            let address = listener.local_addr().unwrap();

            let server = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream.set_read_timeout(Some(TIMEOUT)).unwrap();

                let mut request = String::new();
                let mut reader = BufReader::new(&stream);

                loop {
                    let mut line = String::new();

                    assert!(reader.read_line(&mut line).unwrap() > 0);

                    if line == "\r\n" {
                        break;
                    }

                    request.push_str(&line);
                }

                stream
                    .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
                    .unwrap();

                request
            });

            // The loopback fixture uses HTTP; the production builder keeps
            // HTTPS enforcement for requests and redirects.
            let response = builder(user_agent)
                .https_only(false)
                .no_proxy()
                .build()
                .unwrap()
                .get(format!("http://{address}/"))
                .timeout(TIMEOUT)
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), 204);

            let request = server.join().unwrap();

            let actual_user_agent = request
                .lines()
                .filter_map(|line| line.split_once(':'))
                .find(|(name, _)| name.eq_ignore_ascii_case("user-agent"))
                .map(|(_, value)| value.trim());

            assert_eq!(actual_user_agent, Some(user_agent));
        }
    }
}
