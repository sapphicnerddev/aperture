//! aperture-sys crate for aperture
//!
//! This is the System crate, this handles the unsafe portion of mangling Steamworks into something
//! that Rust can understand. By doing this dynamically, we avoid requiring Steamworks itself to be
//! compiled into the project, saving time and potential legal risks.
//!
//! Coming soon: actual docs...

pub mod loader;
