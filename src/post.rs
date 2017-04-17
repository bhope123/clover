use std::fmt;

use chrono::{DateTime, NaiveDateTime, UTC};

/// A `Post` owns all the data of a post. They are stored in `Vec<Post>` in
/// their respective `Thread`.
///
/// Read more about the Posts object at https://github.com/4chan/4chan-API.
/// Defaults are for optional fields.
#[derive(Clone, Debug, Deserialize)]
pub struct Post {
    pub no: u64,
    pub resto: u64,
    #[serde(default="default::<u8>")]
    pub sticky: u8,
    #[serde(default="default::<u8>")]
    pub closed: u8,
    #[serde(default="default::<u8>")]
    pub archived: u8,
    #[serde(default="default::<u32>")]
    pub archived_on: u32,
    pub now: String,
    pub time: u32,
    #[serde(default="default::<String>")]
    pub name: String,
    #[serde(default="default::<String>")]
    pub trip: String,
    #[serde(default="default::<String>")]
    pub id: String,
    #[serde(default="default::<String>")]
    pub capcode: String,
    #[serde(default="default::<String>")]
    pub country: String,
    #[serde(default="default::<String>")]
    pub country_name: String,
    #[serde(default="default::<String>")]
    pub sub: String,
    #[serde(default="default::<String>")]
    pub com: String,
    #[serde(default="default::<u64>")]
    pub tim: u64,
    #[serde(default="default::<String>")]
    pub filename: String,
    #[serde(default="default::<String>")]
    pub ext: String,
    #[serde(default="default::<u32>")]
    pub fsize: u32,
    #[serde(default="default::<String>")]
    pub md5: String,
    #[serde(default="default::<u16>")]
    pub w: u16,
    #[serde(default="default::<u16>")]
    pub h: u16,
    #[serde(default="default::<u8>")]
    pub tn_w: u8,
    #[serde(default="default::<u8>")]
    pub tn_h: u8,
    #[serde(default="default::<u8>")]
    pub file_deleted: u8,
    #[serde(default="default::<u8>")]
    pub spoiler: u8,
    #[serde(default="default::<u8>")]
    pub custom_spoiler: u8,
    #[serde(default="default::<u16>")]
    pub omitted_posts: u16,
    #[serde(default="default::<u16>")]
    pub omitted_images: u16,
    #[serde(default="default::<u32>")]
    pub replies: u32,
    #[serde(default="default::<u32>")]
    pub images: u32,
    #[serde(default="default::<u8>")]
    pub bumplimit: u8,
    #[serde(default="default::<u8>")]
    pub imagelimit: u8,
    #[serde(default="default::<CapcodeReplies>")]
    pub capcode_replies: CapcodeReplies,
    #[serde(default="default::<i64>")]
    pub last_modified: i64,
    #[serde(default="default::<String>")]
    pub tag: String,
    #[serde(default="default::<String>")]
    pub semantic_url: String,
    #[serde(default="default::<u16>")]
    pub since4pass: u16,

    // Extra field to encompass topic posts in catalog json.
    #[serde(default="default::<Vec<LastReply>>")]
    pub last_replies: Vec<LastReply>,

    // Extra fields to encompass topic posts in thread json.
    #[serde(default="default::<u16>")]
    pub unique_ips: u16,
    #[serde(default="default::<u16>")]
    pub tail_size: u16
}

impl Post {
    /// The If-Modified-Since header requires the date last modified to be in
    /// a specific format as RFC 7232 section 3.3 dictates.
    ///
    /// See `chrono::format::strftime` for the format specifications.
    ///
    /// Returns Some if the post has a last_modified or None if it doesn't.
    pub fn if_modified_since(&self) -> Option<::IfModifiedSince> {
        // If-Modified-Since: Sat, 29 Oct 1994 19:43:31 GMT
        //                    %a,  %d %b  %Y   %T       GMT
        let format = "%a, %d %b %Y %T GMT";

        if self.last_modified == 0 {
            return None
        }

        let dt = DateTime::<UTC>::from_utc(
            NaiveDateTime::from_timestamp(self.last_modified, 0), UTC);

        let fmt_date = dt.format(&format).to_string();

        Some(::IfModifiedSince(fmt_date))
    }

    pub fn is_match(&self, regex: &::regex::Regex) -> bool {
        regex.is_match(&self.name) ||
            regex.is_match(&self.sub) ||
            regex.is_match(&self.com) ||
            regex.is_match(&self.filename)
    }
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no: {}, last_modified: {}", self.no, self.last_modified)
    }
}

/// A `LastReply` is an abridged form of a `Post` given by a catalog.
#[derive(Clone, Debug, Deserialize)]
pub struct LastReply {
    pub no: u64,
    pub now: String,
    #[serde(default="default::<String>")]
    pub name: String,
    #[serde(default="default::<String>")]
    pub com: String,
    pub time: u64,
    pub resto: u64
}

impl Default for LastReply {
    fn default() -> LastReply {
        LastReply {
            no: 0,
            now: String::new(),
            name: String::new(),
            com: String::new(),
            time: 0,
            resto: 0
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CapcodeReplies {
    #[serde(default="default::<Vec<u64>>")]
    admin: Vec<u64>
}

impl Default for CapcodeReplies {
    fn default() -> CapcodeReplies {
        CapcodeReplies { admin: Vec::new() }
    }
}

/// Returns the default of a type that implements `Default`.
fn default<T: Default>() -> T {
    Default::default()
}

#[cfg(test)]
mod test {
    #[test]
    fn post_if_modified_since_test() {
        let post = ::Post {
            no: 0,
            resto: 0,
            sticky: 0,
            closed: 0,
            archived: 0,
            archived_on: 0,
            now: String::new(),
            time: 0,
            name: String::new(),
            trip: String::new(),
            id: String::new(),
            capcode: String::new(),
            country: String::new(),
            country_name: String::new(),
            sub: String::new(),
            com: String::new(),
            tim: 0,
            filename: String::new(),
            ext: String::new(),
            fsize: 0,
            md5: String::new(),
            w: 0,
            h: 0,
            tn_w: 0,
            tn_h: 0,
            file_deleted: 0,
            spoiler: 0,
            custom_spoiler: 0,
            omitted_posts: 0,
            omitted_images: 0,
            replies: 0,
            images: 0,
            bumplimit: 0,
            imagelimit: 0,
            capcode_replies: Default::default(),
            last_modified: 1492218205,
            tag: String::new(),
            semantic_url: String::new(),
            since4pass: 0,
            last_replies: Vec::new(),
            unique_ips: 0,
            tail_size: 0
        };

        let ims = post.if_modified_since().unwrap();
        assert_eq!("Sat, 15 Apr 2017 01:03:25 GMT", &ims.0);
    }
}
