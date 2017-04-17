clover
======

An (Unofficial) Rust library providing a wrapper around the 4chan API.

Usage
-----

For various reasons, I am not exporting this on crates.io

Feel free to [add it as a dependency from git](http://doc.crates.io/specifying-dependencies.html#specifying-dependencies-from-git-repositories)

Example usage:

```rust
extern crate clover;
extern crate env_logger;

use std::cell::RefCell;
use std::rc::Rc;
use env_logger;

fn main() {
    env_logger::init().unwrap();
    let client = Rc::new(RefCell::new(clover::Client::new().unwrap()));
    let g = clover::Board::new(client, "g").unwrap();
    let _ = g.catalog().unwrap();
    let sticky_candidates = g.find_cached("installgentoo")
        .expect("GNU/Linux meme dead");
}
```

See [the provided example](https://github.com/mikopits/clover/examples/find_threads.rs)
for a more detailed explanation on how to use this library.
