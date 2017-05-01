clover
======

An (Unofficial) Rust library providing a wrapper around the 4chan API.
Thread safe and handles API rate throttling for you.

Usage
-----

For various reasons, I am not exporting this on crates.io.

Feel free to [add it as a dependency from git](http://doc.crates.io/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

Example usage:

```rust
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
```

Todos
-----

Feel free to have a go at any of these, I don't have motivation to work on
something that "just werks" for me. Who would use a Rust 4chan library anyway?

*Hard(ish)*
* Any feature not listed here. Feel free to contribute them.
* Implement compatability with the new imageservers (ie. is2.4chan.org).
* Some refactors (marked with TODO in comments).
* Implement filters to not cache threads from your least favourite shitposters.
* Implement optional auto-updating threads on a timer that relaxes when the
thread is stale.
* Allow for regex customization in `find_cached`.

*Easy but annoying*
* Perhaps the Arc and Mutex container shouldn't have to be written manually
and should be handled internally.
* `Post` field types probably aren't the most optimal for memory.
* Write more tests. Preferably ones that don't fail when you're not connected
to the internet.
* Get rid of the time and hyper dependencies.
