use super::super::{Error, List, tests::LIST};
use super::*;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

// Local HTTP fixtures keep the tests independent of GitHub and proxy settings.
#[track_caller]
fn server(responses: Vec<(u16, String)>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());

    let handle = thread::spawn(move || {
        let mut requests = Vec::new();

        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();

            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            let mut reader = BufReader::new(&stream);
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            requests.push(line.trim().into());

            loop {
                line.clear();
                reader.read_line(&mut line).unwrap();

                if line == "\r\n" || line.is_empty() {
                    break;
                }
            }

            write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }

        requests
    });

    (url, handle)
}

#[track_caller]
fn downloader() -> Downloader {
    Downloader::with_client(
        Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap(),
    )
}

#[tokio::test]
async fn downloads_xml_for_borrowed_queries() {
    let (url, server) = server(vec![(200, LIST.into())]);

    let source = downloader()
        .download_list(&format!("{url}/custom/list.xml?version=2"))
        .await
        .unwrap();

    assert_eq!(source, LIST);

    let list = List::new(&source);
    assert!(list.get_for_game("WASM only").unwrap().is_some());
    assert_eq!(list.source().as_ptr(), source.as_ptr());

    assert_eq!(
        server.join().unwrap(),
        ["GET /custom/list.xml?version=2 HTTP/1.1"]
    );
}

#[tokio::test]
async fn download_reports_http_errors_and_defers_parsing() {
    let (url, server) = server(vec![(404, LIST.into())]);

    assert_eq!(
        downloader().download_list(&url).await.unwrap_err().status(),
        Some(reqwest::StatusCode::NOT_FOUND)
    );

    server.join().unwrap();

    let (url, server) = self::server(vec![(200, "<AutoSplitters>".into())]);
    let source = downloader().download_list(&url).await.unwrap();
    assert_eq!(source, "<AutoSplitters>");

    assert!(matches!(
        List::new(&source).get_for_game("Test"),
        Err(Error::Xml { .. })
    ));

    server.join().unwrap();
}

#[tokio::test]
async fn downloads_referenced_module_into_memory() {
    let (url, server) = server(vec![(200, "\0asm\x01\0\0\0".into())]);

    let source = LIST.replace(
        "https://example.com/game.wasm?x=1&amp;y=2",
        &format!("{url}/game%20one.wasm?x=1&amp;y=2"),
    );

    let list = List::new(&source);
    let splitter = list.get_for_game("Game & One").unwrap().unwrap();

    let [url] = splitter.urls() else {
        panic!("expected only the runtime module URL")
    };

    let bytes = downloader().download_file(url).await.unwrap();
    assert_eq!(bytes, b"\0asm\x01\0\0\0");

    assert_eq!(
        server.join().unwrap(),
        ["GET /game%20one.wasm?x=1&y=2 HTTP/1.1"]
    );
}

#[tokio::test]
async fn file_download_reports_http_errors() {
    let (url, server) = server(vec![(503, "unavailable".into())]);

    assert_eq!(
        downloader().download_file(&url).await.unwrap_err().status(),
        Some(reqwest::StatusCode::SERVICE_UNAVAILABLE)
    );

    server.join().unwrap();
}
