use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    pub output: String,

    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    pub delimiter: char,

    // short of header is h which duplicates with h of help, error.
    #[arg(long, help = "Has header or not", default_value_t = true)]
    pub header: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "age")]
    pub age: u32,
    #[serde(rename = "city")]
    pub city: String,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("Input file '{}' does not exist.", file))
    }
}