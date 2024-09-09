// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

mod clients;
mod context;
pub mod headers;
mod models;
mod options;
mod pageable;
mod pipeline;
pub mod policies;
mod request;
mod response;

pub use clients::*;
pub use context::*;
pub use headers::Header;
pub use models::*;
pub use options::*;
pub use pageable::*;
pub use pipeline::*;
pub use policies::*;
pub use request::*;
pub use response::*;

// Re-export derive macro for Model.
#[cfg(feature = "derive")]
pub use typespec_derive::Model;

// Re-export important types.
pub use http_types::{Method, StatusCode};
pub use url::Url;
