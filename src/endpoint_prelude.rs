// Originally from https://gitlab.kitware.com/utils/rust-gitlab
//
// Modified in an attempt to make it general beyond just gitlab

//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`Endpoint`](../trait.Endpoint.html) trait.

pub use std::borrow::Cow;

pub use http::Method;

pub use crate::client::Client;
pub use crate::endpoint::Endpoint;
pub use crate::error::BodyError;
pub use crate::params::FormParams;
pub use crate::params::QueryParams;
