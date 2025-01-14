//! A library to provide additional utilies for Discord bots created with [`serenity`].
//!
//! This library provides implementations to easily:
//! - Convert a string to [`serenity`]'s guild-specific models.
//! - Get user response using message or reaction prompts.
//! - Display paginated reaction-based messages/menus.
//! - Format text in different ways before sending.
//! - Create embeds and messages with field access.
//!
//! See module level documentation for in-depth info about the utilities
//! provided by this crate.
//!
//! ## Installation and Usage
//!
//! To use this crate, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! serenity_utils = "0.5.1"
//! ```
//!
//! **Note:** This crate only supports [`serenity`]'s async versions.
//!
//! ## Examples
//!
//! Examples detailing and explaining usage of the basic functionality of the
//! library can be found in the [`examples`] directory.
//!
//! [`serenity`]: https://github.com/serenity-rs/serenity
//! [`examples`]: https://github.com/AriusX7/serenity-utils/tree/current/examples

pub mod builder;
pub mod conversion;
mod error;
pub mod formatting;
pub mod menu;
pub mod misc;
pub mod prelude;
pub mod prompt;

#[doc(inline)]
pub use error::Error;
