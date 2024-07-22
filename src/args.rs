use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Debug)]
struct Api {
    key: String
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// A name of your city. E.g.: Tokyo
    #[arg(short, long)]
    pub name: String,

    /// You can provide optional state code value. E.g: JP
    #[arg(short, long)]
    pub country: Option<String>
}

// TODO - subcomandy - Api 