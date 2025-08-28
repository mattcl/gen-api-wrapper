// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Originally from https://gitlab.kitware.com/utils/rust-gitlab
//
// Modified in an attempt to make it general beyond just gitlab
//

use std::future::Future;

use http::Uri;
use url::Url;

use crate::{
    client::{AsyncClient, Client},
    error::ApiError,
};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// A trait which represents a query which may be made to a client.
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a client.
pub trait AsyncQuery<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    /// Perform the query asynchronously against the client.
    fn query_async(&self, client: &C)
        -> impl Future<Output = Result<T, ApiError<C::Error>>> + Send;
}
