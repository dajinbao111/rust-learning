use std::str::FromStr;
use anyhow::{anyhow, Result};
use reqwest::Url;
use clap::{Args, Parser};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Args)]
struct Get {
    #[arg(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Debug, Args)]
struct Post {
    #[arg(parse(try_from_str = parse_url))]
    url: String,
    #[arg(parse(try_from_str = parse_kv_pair))]
    body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse().unwrap();
    Ok(s.into())
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: split.next().ok_or_else(err).unwrap().to_string(),
            v: split.next().ok_or_else(err).unwrap().to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse().unwrap())
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts)
}