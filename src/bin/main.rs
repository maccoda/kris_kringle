extern crate kris_kringle;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

use kris_kringle::KrisKringles;

fn main() {
    let matches = App::new("Kris Kringle Allocation")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT").help("Sets the input file to use").index(1).required(true))
        .get_matches();

    let file_name = matches.value_of("INPUT").expect("Need a configuration file!");
    let kks = KrisKringles::build_kks_from_file(file_name);
    kks.write_kks_to_file("test.kk");
}