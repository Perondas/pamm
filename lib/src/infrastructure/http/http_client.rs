use anyhow::Context;
use reqwest::{Client, RequestBuilder, Response};
use std::sync::LazyLock;
use url::Url;

/// One client for the whole process, so connections to the remote are pooled
/// and reused across the many small requests a sync makes.
static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub(crate) fn get(url: &Url) -> RequestBuilder {
    CLIENT.get(url.clone())
}

/// Sends the request, treating any non-success status as an error.
pub(crate) async fn send(request: RequestBuilder, url: &Url) -> anyhow::Result<Response> {
    request
        .send()
        .await
        .with_context(|| format!("Request to {} failed", url))?
        .error_for_status()
        .with_context(|| format!("Request to {} failed", url))
}
