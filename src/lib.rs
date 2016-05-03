extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;
use hyper::error::Error;
use hyper::header::{ContentType, Headers, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel};
use rustc_serialize::json;
use std::io::Read;

static API_URL: &'static str = "https://logs-01.loggly.com/inputs/";

/// This struct contains the API key to authenticate to the Loggly API.
pub struct LogglyClient {
    api_key: String,
}

/// This struct contains fields which comprise the message.
/// Zero or more tags can be added to each message.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct LogglyMessage {
    pub message: String,
    pub from: String,
    pub tags: Vec<String>
}

impl LogglyMessage {
    pub fn new() -> LogglyMessage {
        LogglyMessage {
            message: String::new(),
            from: String::new(),
            tags: vec![]
        }
    }
}

impl LogglyClient {
    /// Create a LogglyClient with the supplied API key.
    pub fn new(key: String) -> LogglyClient {
        LogglyClient {
            api_key: key
        }
    }

    /// Send a JSON-encoded message to Loggly
    pub fn send(self, msg: LogglyMessage) -> Result<String, Error> {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])));
        headers.set(UserAgent("loggly-rs".to_owned()));

        // Serialise the message using `json::encode`
        let encoded = json::encode(&msg).unwrap();
        // println!("JSON encoded: {:?}", encoded);

        // Build the target Loggly URL with the API URL and the API Key
        let target_url = String::from(API_URL) + &self.api_key;

        // Post the message to Loggly
        let mut result = try!(client.post(&target_url)
                            .headers(headers)
                            .body(&encoded)
                            .send());

        // Return the body of the result from the post action
        let mut body = String::new();
        try!(result.read_to_string(&mut body));
        Ok(body)

    }
}
