//! # faceit
//!
//! A Rust client library for the [FACEIT Public API](https://developers.faceit.com/).
//!
//! This crate provides a type-safe, async API client with comprehensive error handling,
//! builder pattern configuration, and full Data API v4 support.
//!
//! ## Features
//!
//! - **Type-safe API**: All API responses are deserialized into strongly-typed Rust structs
//! - **Async/await support**: Built on `tokio` and `reqwest` for async operations
//! - **Builder pattern**: Flexible client configuration via [`HttpClientBuilder`]
//! - **Comprehensive error handling**: Detailed error types for all failure modes
//! - **Ergonomic APIs**: Optional wrapper types for convenient resource access (enable `ergonomic` feature)
//! - **Full API coverage**: Supports all Data API v4 endpoints
//!
//! ## Quick Start
//!
//! ```no_run
//! use faceit::HttpClient;
//!
//! # async fn example() -> Result<(), faceit::error::Error> {
//! // Create a client
//! let client = HttpClient::new();
//!
//! // Get player by ID
//! let player = client.get_player("player-id-here").await?;
//! println!("Player: {}", player.nickname);
//! # Ok(())
//! # }
//! ```
//!
//! ## Authentication
//!
//! The FACEIT API supports both API keys and OAuth2 access tokens. Both are configured
//! using the [`HttpClientBuilder::api_key`] method:
//!
//! ```no_run
//! use faceit::HttpClient;
//!
//! # fn example() -> Result<(), faceit::error::Error> {
//! let client = HttpClient::builder()
//!     .api_key("your-api-key-or-access-token")
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! See the [README](https://github.com/muijf/faceit) for more examples and documentation.
//!
//! ## Modules
//!
//! - [`error`] - Error types for API operations
//! - [`http`] - HTTP client and builder types
//! - [`types`] - API response types

pub mod error;
pub mod http;
pub mod types;

pub use http::{Client as HttpClient, ClientBuilder as HttpClientBuilder};
