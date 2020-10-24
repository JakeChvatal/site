#[macro_use]
extern crate slugify;
extern crate dotenv;

use std::env;
use std::fs;

use std::convert::From;
use std::io::{Error as IOError, Write};
use std::string::FromUtf8Error;

use orgize::export::{DefaultHtmlHandler, HtmlHandler};
use orgize::{Element, Org};

use serde::{Deserialize, Serialize};
use slugify::slugify;

use github_gql as gh;
use gh::client::Github;
use gh::query::Query;
use serde_json::Value;

use dotenv::dotenv;

fn get_user_info() {
  dotenv().ok();

  let vars = env::vars().filter(|(key, value)| key == "GITHUB_API_KEY").collect::<Vec<(std::string::String, std::string::String)>>();

  let gh_token = env::var("GITHUB_API_KEY").unwrap();

  let mut g = Github::new(&gh_token).unwrap();
  let (headers, status, json) = g.query::<Value>(&Query::new_raw("query {viewer {login} }")).unwrap();

  println!("{:#?}", headers);
  println!("{}", status);
}

fn main() {
  get_user_info();
}

/// Parse an org-mode document into its Org representation
fn parse() {}

    // let args: Vec<String> = env::args().collect();
    // let filename = "test.txt";
//
    // println!("in file {}", filename);
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
//
    // let contents = Org::parse(&contents);
    // let mut writer = Vec::new();
//
    // contents.write_html(&mut writer).unwrap();
//
    // println!("{}", String::from_utf8(writer).unwrap());
    // println!("{}", serde_json::to_string(&contents).unwrap());

// GQL Query for getting pinned repositories off of repo
// How many will I want eventually?
// query {
    // user(login:"jakechv") {
    // pinnedItems(first: 6, types: [REPOSITORY]) {
      // totalCount
      // edges {
        // node {
          // ... on Repository {
            // name
            // owner {
              // id
              // login
            // }
            // url
            // description
          // }
        // }
      // }
    // }
  // }
// }
