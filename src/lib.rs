#![doc = include_str!("../README.md")]
use http::{header, Request};

use crate::{
    client::{AsyncClient, Client},
    endpoint_prelude::Endpoint,
    error::ApiError,
    query::{AsyncQuery, Query},
};

// Most of this module comes from the core api implementation of
// https://gitlab.kitware.com/utils/rust-gitlab. Modifications were made to make
// this more genralized instead of gitlab-specific
pub mod client;
pub mod endpoint;
pub mod endpoint_prelude;
pub mod error;
pub mod params;
pub mod query;

/// A query modifier that returns the raw data from the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<E> {
    endpoint: E,
}

/// Return the raw data from the endpoint.
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw { endpoint }
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest(req, data)?;
        if !rsp.status().is_success() {
            return Err(ApiError::server_error(rsp.status(), rsp.body()));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}

impl<E, C> AsyncQuery<Vec<u8>, C> for Raw<E>
where
    E: Endpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let rsp = client.rest_async(req, data).await?;
        if !rsp.status().is_success() {
            return Err(ApiError::server_error(rsp.status(), rsp.body()));
        }

        Ok(rsp.into_body().as_ref().into())
    }
}
