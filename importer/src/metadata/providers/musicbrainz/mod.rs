pub mod client;
pub mod entities;
pub mod error;

pub use client::{Client, SearchResponse, SearchResult};
pub use error::Error;

pub use crate::utils::lev::damlev;
