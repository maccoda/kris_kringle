extern crate kris_kringle;

extern crate rand;
#[macro_use]
extern crate clap;

use std::io::{self, Write};
use std::fs::File;

use clap::{App, Arg};



fn main() {

    let matches = App::new("Kris Kringle Allocation")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("stdin")
                 .long("stdin")
                 .help("Accepts the participants through terminal")
                 .takes_value(false))
        .arg(Arg::with_name("stdout")
                 .long("out")
                 .help("Allows for printing of results to terminal")
                 .takes_value(false))
        .arg(Arg::with_name("INPUT").help("Sets the input file to use").index(1))
        .get_matches();

    let use_stdin = matches.is_present("stdin");
    let use_stdout = matches.is_present("stdout");

    let mut pairs = vec![];

    if use_stdin {
        println!("How many people are part of this KK group?");
        let mut all: Vec<String> = vec![];
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed read");
        let num = num.trim().parse().expect("Failed parse");
        for _ in 0..num {
            println!("Give me a name: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed read");
            all.push(name);

            pairs = kris_kringle::assign_kks(&all);
        }
    } else {
        println!("Reading from file");
        let file_name =
            matches.value_of("INPUT").expect("Use --stdin if do not have a file to use");
        pairs = kris_kringle::assign_kks_from_file(file_name);
    }
    // Create the output for the results
    for pair in &pairs {
        let mut file_name: String = pair.get_giver().get_name();
        file_name.push_str(".kk");
        let mut file = File::create(file_name).unwrap();
        let content: &[u8] = &(pair.get_receiver().get_name().into_bytes())[..];
        file.write_all(content).unwrap();
    }

    // Time to ask who is doing this
    if use_stdout {
        loop {
            println!("Who are you exactly???");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("No name provided");
            let name: String = name.trim().to_string();
            match kris_kringle::find_kk(&pairs, &name) {
                Some(rec) => println!("Give to {}", rec),
                None => println!("You are not welcome here {}!!!", name),
            }
            println!("Got it?");
            let mut name = String::new();
            // Don't actually care what they write it is clearing
            io::stdin().read_line(&mut name).expect("No input given");
            // Clears the terminal screen
            print!("{}[2J", 27 as char);
        }
    }

}
