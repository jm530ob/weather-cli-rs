use serde::Deserialize;
use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command_type: CommandType
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// Ensure you have entered a valid API key before continuing 
    Key(KeyCommand),
    /// Setup your city
    Set(SetCommand)

}

#[derive(Args, Debug)]
pub struct KeyCommand {
    pub api_key: Option<String>
}

#[derive(Args, Debug, Deserialize)]
pub struct SetCommand {
    /// A name of your city. E.g.: Tokyo
    #[arg(short, long)]
    pub name: String,
    
    /// You can provide optional state code value. E.g: JP
    #[arg(short, long)]
    pub country: Option<String>,
}
