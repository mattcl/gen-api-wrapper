// Originally from https://gitlab.kitware.com/utils/rust-gitlab
//
// Modified in an attempt to make it general beyond just gitlab

use std::any;
use std::error::Error;

use thiserror::Error;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// JSON deserialization from API failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Server returned an error message.
    #[error("server responded with error: {}", msg)]
    Server {
        /// The error message from the server.
        msg: String,
    },
    /// Server returned an error without JSON information.
    #[error("server responded with error: {} - {}", .status, String::from_utf8_lossy(.data))]
    ServerService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The error data from the server.
        data: Vec<u8>,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }

    /// Wrap a client error in another wrapper.
    pub fn map_client<F, W>(self, f: F) -> ApiError<W>
    where
        F: FnOnce(E) -> W,
        W: Error + Send + Sync + 'static,
    {
        match self {
            Self::Client { source } => ApiError::client(f(source)),
            Self::UrlParse { source } => ApiError::UrlParse { source },
            Self::Body { source } => ApiError::Body { source },
            Self::Json { source } => ApiError::Json { source },
            Self::Server { msg } => ApiError::Server { msg },
            Self::ServerService { status, data } => ApiError::ServerService { status, data },
            Self::DataType { source, typename } => ApiError::DataType { source, typename },
        }
    }

    pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
        Self::ServerService {
            status,
            data: body.into_iter().copied().collect(),
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}
