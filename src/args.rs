use clap::{Args, Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command_type: CmdType
}

#[derive(Subcommand, Debug)]
pub enum CmdType {
    /// Ensure you have entered a valid API key before continuing 
    Key(ApiKey),

    /// Sets up your city
    Set(City),

    /// Executes the app using the stored configuration
    Go
}

#[derive(Args, Debug)]
pub struct ApiKey {
    pub api_key: Option<String>
}

#[derive(Args, Debug)]
pub struct City {
    /// A name of your city. E.g.: Tokyo
    #[arg(short, long)]
    pub name: String,
    
    /// You can provide optional state code value. E.g: JP
    #[arg(short, long)]
    pub country: Option<String>,
}