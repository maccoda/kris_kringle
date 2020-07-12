#[macro_use]
extern crate clap;
extern crate kris_kringle;
extern crate log;

use clap::{App, Arg};

use kris_kringle::KrisKringles;

fn main() {
    let matches = App::new("Kris Kringle Allocation")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("email")
                .help(
                    "When set will send emails using
            envionment variables",
                )
                .short("e")
                .long("email")
                .takes_value(false),
        )
        .get_matches();

    log::set_logger(|max_log_level| {
        max_log_level.set(::log::LogLevelFilter::Debug);
        Box::new(kris_kringle::kk_log::SimpleLogger)
    }).unwrap();

    let file_name = matches
        .value_of("INPUT")
        .expect("Need a configuration file!");
    let kks = KrisKringles::build_kks_from_file(file_name);
    kks.write_kks_to_file("test.kk");
    if matches.value_of("email").is_some() {
        kks.email_givers().unwrap();
    }
}