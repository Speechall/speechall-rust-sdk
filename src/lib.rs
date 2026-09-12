//! Rust client and models for the [Speechall API](https://docs.speechall.com).
//!
//! This crate is generated from Speechall's shared OpenAPI document. Configure
//! the client with an API key, then call an operation:
//!
//! ```no_run
//! use speechall::ClientBuilder;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = ClientBuilder::new()
//!     .bearer_auth(std::env::var("SPEECHALL_API_KEY")?)
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! Run `./regenerate.sh` after changing the shared OpenAPI document. Generated
//! modules are committed for reproducible SDK releases and must not be edited
//! by hand.

#[path = "generated/client.rs"]
pub mod client;
#[path = "generated/models.rs"]
pub mod models;
#[path = "generated/views.rs"]
pub mod views;

pub use client::ClientBuilder;
