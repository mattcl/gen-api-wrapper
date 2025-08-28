// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Originally from https://gitlab.kitware.com/utils/rust-gitlab
//
// Modified in an attempt to make it general beyond just gitlab

use std::error::Error;
use std::future::Future;

use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

use crate::error::ApiError;

/// A trait representing a client which can communicate with a generic instance via REST.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with a generic instance.
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with a generic instance.
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> impl Future<Output = Result<Response<Bytes>, ApiError<Self::Error>>> + Send;
}
