use crate::run::{parser::composite, saver};
use crate::{Run, Timer};
use snafu::ResultExt;
use std::io::Cursor;

// TODO: We probably don't actually want any tests. These just cause trouble on
// various levels:
// - dev-dependencies can't be optional
// - dev-dependency features leak into normal dependencies
// - failing tests when you are not connected to the internet
// - spurious failures every now and then
// Maybe these should be optional tests outside of the cargo workspace.
#[cfg(test)]
mod tests;

pub use api::run::UploadedRun;
pub use api::Client;
pub use api::Error as UploadError;
pub use splits_io_api as api;

#[derive(Debug, snafu::Snafu)]
pub enum DownloadError {
    /// Failed to download the run.
    Download { source: api::Error },
    /// Failed to parse the run.
    Parse { source: composite::Error },
}

pub async fn download_run(
    client: &Client,
    id: &str,
) -> Result<composite::ParsedRun, DownloadError> {
    let bytes = api::run::download(client, id).await.context(Download)?;
    let bytes: &[u8] = &*bytes;
    composite::parse(Cursor::new(bytes), None, false).context(Parse)
}

pub async fn upload_run(client: &Client, run: &Run) -> Result<UploadedRun, UploadError> {
    api::run::upload_lazy(client, |writer| saver::livesplit::save_run(run, writer)).await
}

pub async fn upload_timer(client: &Client, timer: &Timer) -> Result<UploadedRun, UploadError> {
    api::run::upload_lazy(client, |writer| saver::livesplit::save_timer(timer, writer)).await
}
