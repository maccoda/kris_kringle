extern crate kris_kringle;
#[macro_use]
extern crate clap;
extern crate log;

use clap::{App, Arg};

use kris_kringle::KrisKringles;

fn main() {
    let matches = App::new("Kris Kringle Allocation")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("INPUT").help("Sets the input file to use").index(1).required(true))
        .get_matches();

    log::set_logger(|max_log_level| {
                        max_log_level.set(::log::LogLevelFilter::Debug);
                        Box::new(SimpleLogger)
                    })
            .unwrap();

    let file_name = matches.value_of("INPUT").expect("Need a configuration file!");
    let kks = KrisKringles::build_kks_from_file(file_name);
    kks.write_kks_to_file("test.kk");
}

use log::{LogLevel, LogRecord, LogMetadata};

struct SimpleLogger;

impl ::log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
