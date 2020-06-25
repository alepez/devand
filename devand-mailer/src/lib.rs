mod api;

#[cfg(feature = "client")]
mod client;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
mod mailer;

#[cfg(feature = "client")]
pub use client::{Client, ClientConf};

#[cfg(feature = "server")]
pub use server::{Server, ServerConf};
