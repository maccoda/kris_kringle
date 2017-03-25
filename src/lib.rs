extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;

use rand::Rng;

mod conf;
mod file_utils;

#[derive(Debug)]
pub struct KkPair {
    giver: conf::Participants,
    receiver: conf::Participants,
}

impl KkPair {
    pub fn get_giver(&self) -> conf::Participants {
        self.giver.clone()
    }
    pub fn get_receiver(&self) -> conf::Participants {
        self.receiver.clone()
    }
}

#[derive(Debug)]
pub struct KrisKringles {
    configuration: conf::KkConf,
    pairs: Vec<KkPair>,
}

impl KrisKringles {
    pub fn build_kks_from_file<P: AsRef<Path>>(path: P) -> KrisKringles {
        let conf = conf::KkConf::build(path);
        let pairs = perform_pairing(&conf.get_participants());

        KrisKringles {
            configuration: conf,
            pairs: pairs,
        }

    }

    pub fn write_kks_to_file<P: AsRef<Path>>(&self, path: P) {
        let mut all_content = String::new();
        for pair in &self.pairs {
            let mut file_name: String = pair.get_giver().get_name();

            all_content.push_str(&file_name);
            all_content.push_str(" --> ");
            all_content.push_str(&pair.get_receiver().get_name());
            all_content.push_str("\n");

            file_name.push_str(".kk");
            file_utils::write_to_file(file_name, pair.get_receiver().get_name());
        }
        file_utils::write_to_file(path, all_content);
    }

    /// Given the name of the giver will find the name of the receiver
    pub fn find_kk(&self, giver: &str) -> Option<String> {
        for pair in &self.pairs {
            if pair.giver.get_name().eq(giver) {
                return Some(pair.receiver.get_name().clone());
            }
        }
        None
    }
}

/// Performs the pairing of each giver to the receiver
fn perform_pairing(all: &Vec<conf::Participants>) -> Vec<KkPair> {
    let mut pairs: Vec<KkPair> = Vec::new();
    for person in all {
        pairs.push(KkPair {
                       giver: person.clone(),
                       receiver: person.clone(),
                   });
    }
    while invalid_map(&pairs) {
        shuffle_pairs(all.len(), &mut pairs);
    }
    for pair in &pairs {
        println!("{:?} --> {:?}", pair.giver, pair.receiver);
    }
    pairs
}

/// Mutates the pairs vector by randomly shuffling the receivers for two givers in the
/// provided vector.
fn shuffle_pairs(max_length: usize, pairs: &mut Vec<KkPair>) {
    let giver1_index = rand::thread_rng().gen_range(0, max_length);
    let giver2_index = rand::thread_rng().gen_range(0, max_length);
    let temp = pairs[giver1_index].receiver.clone();
    pairs[giver1_index].receiver = pairs[giver2_index].receiver.clone();
    pairs[giver2_index].receiver = temp;
}

/// Determines if this is a valid match up. Initially needs to check that
/// user is not giving present to themselves. Second check, if enabled is to
/// confirm that groups are different.
fn invalid_map(pairs: &Vec<KkPair>) -> bool {
    for pair in pairs {
        if pair.giver.get_name().eq(&pair.receiver.get_name()) {
            let giver_group = pair.giver.get_group();
            let recvr_group = pair.receiver.get_group();
            if giver_group == recvr_group {
                return true;
            }
        }
    }
    false
}
