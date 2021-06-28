use std::io::Read;
use std::thread::sleep;

use chrono::{DateTime, Duration, UTC};
use serde_json::Value;
use reqwest::header::{Headers, UserAgent};

static BOARDS_URL: &'static str = "https://a.4cdn.org/boards.json";

/// A `Client` makes all the API GET requests. All requests are throttled by
/// a 1 second interval to comply with the 4chan API rules. Use the same client
/// for all your boards (see examples).
#[derive(Debug)]
pub struct Client {
    reqwest_client: ::reqwest::Client,
    all_boards: Vec<String>,
    // List of blue boards
    sfw_boards: Vec<String>,
    // List of red boards
    nsfw_boards: Vec<String>,
    last_request: DateTime<UTC>,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new() -> ::Result<Client> {
        let client = try!(::reqwest::Client::new());

        let last_request = UTC::now();
        let mut res = try!(client.get(BOARDS_URL).send());
        assert!(res.status().is_success());

        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));

        let v: Value = try!(::serde_json::from_str(&buf));
        let mut sfw_boards = Vec::new();
        let mut nsfw_boards = Vec::new();
        let mut all_boards = Vec::new();

        let boards = v["boards"].as_array().unwrap();
        for board in boards {
            let name = board["board"].as_str().unwrap();
            if board["ws_board"].as_u64().unwrap() == 1 {
                sfw_boards.push(name.to_string());
            } else{
                nsfw_boards.push(name.to_string());
            }
            all_boards.push(name.to_string());
        }

        Ok(Client{
            reqwest_client: client,
            all_boards: all_boards,
            sfw_boards: sfw_boards,
            nsfw_boards: nsfw_boards,
            last_request: last_request,
        })
    }

    /// Makes a GET request to the url. Adds an "If-Modified-Since" header if
    /// provided.
    pub fn get(&mut self, url: &str, headers: Option<::IfModifiedSince>)
        -> ::Result<::reqwest::Response> {
        // Throttle so that we make no more than 1 request per second.
        let diff = UTC::now().signed_duration_since(self.last_request);
        if diff < Duration::seconds(1) {
            sleep(try!(diff.to_std()));
        }

        let mut req_headers = Headers::new();
        req_headers.set(UserAgent("clover-rs".to_string()));
        if headers.is_some() {
            for header in headers {
                req_headers.set(header);
            }
        }

        debug!("[{:?}] Making request to url: {} with headers: {:?}",
               UTC::now(), url, req_headers);

        let res = try!(self.reqwest_client.get(url)
                           .headers(req_headers)
                           .send());

        self.last_request = UTC::now();

        Ok(res)
    }

    pub fn is_sfw(&self, name: &str) -> bool {
        self.sfw_boards.contains(&name.to_string())
    }

    pub fn is_nsfw(&self, name: &str) -> bool {
        self.nsfw_boards.contains(&name.to_string())
    }

    pub fn is_valid_board(&self, name: &str) -> bool {
        self.all_boards.contains(&name.to_string())
    }
    
    pub fn all_boards(&self) -> Vec<String> {
        self.all_boards.clone()
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    #[test]
    fn get_board() {
        let client = Arc::new(Mutex::new(::Client::new().unwrap()));
        let g = ::Board::new(client, "g").unwrap();
        let _ = g.catalog().unwrap();
        assert!(g.thread_cache.lock().unwrap().threads.len() > 0);
        let sticky_candidates = g.find_cached("installgentoo")
            .expect("Found no matches for installgentoo");
        assert!(sticky_candidates.len() > 0);
    }
}

