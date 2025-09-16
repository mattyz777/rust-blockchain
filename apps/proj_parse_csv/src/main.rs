/**
 * # add dependency
 * cargo add clap --features=derive
 * 
 * # usage
 * rcli csv -i input.csv -o output.json --header -d ','
 * 
 * # run
 * cd apps/proj_parse_csv
 * cargo run -- csv -i input.csv
 * 
 **/
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help = "Input file path")]
    input: String,

    #[arg(short, long, help = "Output file path", default_value = "output.json")]
    output: String,

    #[arg(short, long, help = "Delimiter character", default_value_t = ',')]
    delimiter: char,

    // short of header is h which duplicates with h of help, error.
    #[arg(long, help = "Has header or not", default_value_t = true)]
    header: bool,
}

fn main() {
    println!("Hello, world!");
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
