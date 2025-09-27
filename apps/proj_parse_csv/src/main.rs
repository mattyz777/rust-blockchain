/**
 * # add dependency
 * cargo add clap --features=derive
 * cargo add csv
 * cargo add serde --features derive
 * 
 * # usage
 * rcli csv -i input.csv -o output.json --header -d ','
 * 
 * # run
 * cd apps/proj_parse_csv
 * 
 * # cargo help cmd
 * cargo run --help
 *
 * # app help cmd due to clap
 * cargo run -- help
 * 
 * # subcmd help cmd due to clap
 * cargo run -- csv -i input.csv
 * cargo run -- csv --input input.csv
 * 
 * --------------------------------------
 * #[arg(short, long, help = "Input file path", value_parser = verify_input_file, default_value = "input.csv")]
 * 
 * 
 * 
 * rust builds two crates: lib and binary in current folder structure
 * `use crate::opts::Opts;` => the compiler looks for a module named opts in the binary crate (which only contains main.rs) ⇒ Not found ⇒ error.
 * ----------------------
 * apps/
 *   proj_parse_csv/
 *     src/
 *       lib.rs
 *       main.rs
 *     Cargo.toml
 * ----------------------
 * 
 * 
 * Package: rcli (from Cargo.toml)
 * ├── Library crate (root: src/lib.rs)
 * │     pub mod opts;
 * │
 * └── Binary crate  (root: src/main.rs)
 *       use rcli::opts;   // OK
 *       use crate::opts;  // ❌ (no opts in this crate)
 * 
 **/
// use clap::Parser;
// use csv::Reader;
// use crate::opts::{Opts, SubCommand, Record}; // not working
// use rcli::opts::{Opts, SubCommand, Record};


fn main() {
    for i in 0..10 {
        println!("Number: {}", i);
    }
}



// fn main1() {
//     let mut ss = String::from("aaa");
//     // let ss1 = &mut ss;
//     let ss2 = &mut ss;

//     ss.push_str("bbb");
//     println!("{} {}", "ss1", ss2);


//     println!("Hello, world!");
//     let opts: Opts = Opts::parse();
//     println!("{:?}", opts);
//     let mut r = Vec::<Record>::new();
//     match opts.cmd {
//         SubCommand::Csv(opts) => {
//             println!("{:?}", opts);
//             let mut reader = Reader::from_path(opts.input).unwrap();
            
//             // collection and panic on error
//             // let records = reader
//             //     .deserialize()
//             //     .map(|record| record.unwrap())
//             //     .collect::<Vec<Record>>();

//             // println!("{:?}", records);

//             reader.deserialize().for_each(|record| r.push(record.unwrap()));
//         }
//     }
//     println!("----------------------------------");
//     println!("{:?}", r);
// }
