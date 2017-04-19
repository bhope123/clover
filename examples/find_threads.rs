extern crate clover;
extern crate env_logger;

use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init().unwrap();

    // Wrap the client in `Arc` and `Mutex`. Use the same client for any and
    // all threads you use (It will automatically throttle as per the API
    // rules). Also, it wraps around a `reqwest` client which also recommends
    // to use the same client for all requests.
    let client = Arc::new(Mutex::new(clover::Client::new().unwrap()));
    let g = clover::Board::new(client, "g").unwrap();

    // Initialize the cache by sweeping the board in one request. The catalog
    // struct is not too useful, but the option is there to use it. Run
    // `catalog` again if you wish to update your thread cache.
    let _ = g.catalog().unwrap();

    // Threads are lazily updated with `find_cached`.
    let mut sticky_candidates = g.find_cached("installgentoo")
        .expect("GNU meme dead");

    // Some time passes. Want to update threads again. Note that each thread
    // is throttled to only be able to send update requests every 10 seconds.
    // This is the absolute minimum required by the API. This is done by
    // sleeping the thread until the time is up, so don't call it at intervals
    // any less than 10 seconds.
    for mut candidate in &mut sticky_candidates {
        candidate.update().expect("Failed to update");
    }
}
