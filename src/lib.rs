#![deny(warnings)]

extern crate chrono;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

pub use self::board::Board;
pub use self::client::Client;
pub use self::error::{Error, Result};
pub use self::post::{LastReply, Post};
pub use self::thread::{Thread, ThreadCache, ThreadDeserializer};

mod board;
mod client;
mod error;
mod post;
mod thread;

/// Define a custom If-Modified-Since header because we use `chrono::time`
/// instead of `time:Tm` and handle date formatting with `chrono`.
header! { (IfModifiedSince, "If-Modified-Since") => [String] }
