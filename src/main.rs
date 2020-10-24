use anyhow::*;
use graphql_client::*;
use log::*;
use prettytable::*;
use serde::*;
use structopt::StructOpt;

type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query_1.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "repository")]
    repo: String,
}

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config: Env = envy::from_env().context("while reading from environment")?;

    let args = Command::from_args();

    let repo = args.repo;
    let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));

    let q = RepoView::build_query(repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&q)
        .send()?;

    let response_body: Response<repo_view::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars: Option<i64> = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count);

    println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "issue", "comments"));

    for issue in &response_data
        .repository
        .expect("missing repository")
        .issues
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(issue) = issue {
            table.add_row(row!(issue.title, issue.comments.total_count));
        }
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }
}

// extern crate dotenv;
// extern crate slugify;

// use std::env;
// use std::fs;

// use std::convert::From;
// use std::io::{Error as IOError, Write};
// use std::string::FromUtf8Error;

// use orgize::export::{DefaultHtmlHandler, HtmlHandler};
// use orgize::{Element, Org};

// use serde::{Deserialize, Serialize};
// use slugify::slugify;

// use serde_json::Value;

// use dotenv::dotenv;

// fn get_user_info() {
//     dotenv().ok();

//     // let vars = env::vars()
//     //     .filter(|(key, value)| key == "GITHUB_API_KEY")
//     //     .collect::<Vec<(std::string::String, std::string::String)>>();

//     let gh_token = env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN env var must be set");

//     let username = "jakechv";

//     use cynic::http::ReqwestBlockingExt;

//     reqwest::blocking::Client::new()
//         .post("https://api.github.com/graphql")
//         .header("Authorization", format!("Bearer {}", gh_token))
//         .header("User-Agent", username)
//         .run_graphql(query)
//         .unwrap();

//     // let query_string = "query {viewer {login} }";
//     let query_string = "query {
//     user(login:\"jakechv\") {
//         pinnedItems(first: 6, types: [REPOSITORY]) {
//             totalCount
//                 edges {
//                     node {
//                     ... on Repository {
//                         name
//                         owner {
//                             id
//                             login
//                         }
//                         url
//                         description
//                     }
//                 }
//             }
//         }
//     }
// }";

//     // println!("{:#?}", headers);
//     // println!("{}", status);
// }

// fn main() {
//     get_user_info();
// }

// /// Parse an org-mode document into its Org representation
// fn parse() {}

// // let args: Vec<String> = env::args().collect();
// // let filename = "test.txt";
// //
// // println!("in file {}", filename);
// // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
// //
// // let contents = Org::parse(&contents);
// // let mut writer = Vec::new();
// //
// // contents.write_html(&mut writer).unwrap();
// //
// // println!("{}", String::from_utf8(writer).unwrap());
// // println!("{}", serde_json::to_string(&contents).unwrap());

// // GQL Query for getting pinned repositories off of repo
// // How many will I want eventually?
