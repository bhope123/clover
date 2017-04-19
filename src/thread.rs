use std::collections::HashMap;
use std::io::Read;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use chrono::{DateTime, Duration, UTC};
use reqwest::StatusCode;

/// A `Thread` is a 4chan thread. Its topic is the OP `Post` and its replies
/// are every reply in the thread.
#[derive(Clone, Debug)]
pub struct Thread {
    pub board_name: String,
    client: Arc<Mutex<::Client>>,
    pub topic: ::Post,
    pub replies: Vec<::Post>,
    pub expired: bool,
    wants_update: bool,
    last_reply_no: u64,
    last_updated: Option<DateTime<UTC>>
}

impl Thread {
    /// Creates a new `Thread` from a topic `Post`.
    pub fn from_topic(post: ::Post,
                      board_name: &str,
                      client: Arc<Mutex<::Client>>) -> Thread {
        Thread {
            board_name: board_name.to_string(),
            client: client,
            topic: post.clone(),
            replies: Vec::new(),
            expired: false,
            wants_update: true,
            last_reply_no: {
                if !post.last_replies.is_empty() {
                    post.last_replies.last().unwrap().no
                } else {
                    0
                }
            },
            last_updated: None
        }
    }

    /// Creates a new `Thread` from a `ThreadDeserializer`.
    pub fn from_deserializer(deserializer: ThreadDeserializer,
                             board_name: &str,
                             client: Arc<Mutex<::Client>>) -> Thread {
        let topic = deserializer.posts.first().unwrap().to_owned();

        Thread {
            board_name: board_name.to_string(),
            client: client,
            topic: topic.clone(),
            replies: deserializer.posts.iter().skip(1).cloned().collect(),
            expired: false,
            wants_update: true,
            last_reply_no: {
                if !topic.last_replies.is_empty() {
                    topic.last_replies.last().unwrap().no
                } else {
                    0
                }
            },
            last_updated: None
        }
    }

    /// Updates a `Thread`, throttling updates by 10 second intervals and
    /// using "If-Modified-Since".
    pub fn update(&mut self) -> ::Result<()> {
        if self.expired { return Ok(()) }

        // Threads should be updated no faster than every 10 seconds.
        if self.last_updated.is_some() {
            let diff = UTC::now()
                .signed_duration_since(self.last_updated.unwrap());
            if diff < Duration::seconds(10) {
                sleep(try!(diff.to_std()));
            }
        }

        let mut res = try!(self.client.lock().unwrap().get(
                &format!("https://a.4cdn.org/{}/thread/{}.json",
                         self.board_name, self.topic.no),
                         self.topic.if_modified_since()));

        self.last_updated = Some(UTC::now());

        match *res.status() {
            StatusCode::Ok => {
                self.wants_update = true;
                let mut buf = String::new();
                try!(res.read_to_string(&mut buf));

                debug!("Got response: {}", buf);

                let thread: ThreadDeserializer = try!(
                    ::serde_json::from_str(&buf));
                self.topic = thread.posts.first().unwrap().to_owned();

                if self.topic.replies > 0 {
                    if self.last_reply_no != 0 {
                        for post in thread.posts.iter().skip(1) {
                            if post.no > self.last_reply_no {
                                self.replies.push(post.to_owned());
                            }
                        }
                    } else {
                        for post in thread.posts.iter().skip(1) {
                            self.replies.push(post.to_owned());
                        }
                    }
                    if self.replies.len() > 0 {
                        self.last_reply_no = self.replies.last().unwrap().no;
                    }
                }
                Ok(())
            },
            StatusCode::NotModified => {
                Ok(())
            },
            StatusCode::NotFound => {
                self.expired = true;
                self.wants_update = false;
                // TODO: Delete from cache? If so, now?
                Ok(())
            }
            _ => Err(::Error::UnexpectedResponse)
        }
    }

    pub fn is_match(&self, regex: &::regex::Regex) -> bool {
        self.topic.is_match(regex)
    }

    pub fn is_expired(&self) -> bool {
        self.expired
    }

    pub fn wants_update(&self) -> bool {
        self.wants_update
    }

    pub fn last_reply(&self) -> Option<::LastReply> {
        match self.topic.last_replies.last() {
            Some(r) => Some(r.clone()),
            None => None
        }
    }

    pub fn url(&self) -> String {
        format!("https://boards.4chan.org/{}/thread/{}",
                &self.board_name, &self.topic.no)
    }

    /// Get a `Vec` of all the image urls in the thread.
    pub fn image_urls(&self) -> Vec<String> {
        let mut images: Vec<String> = Vec::new();
        let topic_img = self.topic.image_url(&self.board_name);
        if topic_img.is_some() {
            images.push(topic_img.unwrap());
        }
        for reply in &self.replies {
            match reply.image_url(&self.board_name) {
                Some(i) => images.push(i),
                None => (),
            }
        }
        images
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThreadDeserializer {
    pub posts: Vec<::Post>
}

/// A `ThreadCache` is an abstraction around a `HashMap<u64, Thread>`.
#[derive(Debug)]
pub struct ThreadCache {
    pub threads: HashMap<u64, Thread>
}

impl ThreadCache {
    pub fn new() -> ThreadCache {
        ThreadCache { threads: HashMap::new() }
    }

    pub fn get(&self, thread_no: u64) -> Option<&Thread> {
        self.threads.get(&thread_no)
    }

    pub fn insert(&mut self, thread: Thread) {
        self.threads.entry(thread.topic.no).or_insert(thread);
    }

    pub fn contains(&self, thread_no: u64) -> bool {
        self.threads.contains_key(&thread_no)
    }

    pub fn remove(&mut self, thread_no: u64) {
        self.threads.remove(&thread_no);
    }
}

impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no: {}, last_modified: {}, expired: {}",
               self.topic.no, self.topic.last_modified, self.expired)
    }
}

impl fmt::Display for ThreadCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "threads: {}", self.threads.keys()
               .map(|k| k.to_string())
               .collect::<Vec<String>>()
               .join(", "))
    }
}
