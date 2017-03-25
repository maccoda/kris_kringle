extern crate kris_kringle;

use kris_kringle::KrisKringles;

fn main() {
    let kks = KrisKringles::build_kks_from_file("tests/resources/full.toml");
    kks.write_kks_to_file("test.kk");
}