extern crate rand;
extern crate toml;

use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;

use rand::Rng;

mod conf;
mod file_utils;

#[derive(Debug)]
pub struct KkPair {
    giver: Person,
    receiver: Person,
}

impl KkPair {
    pub fn get_giver(&self) -> Person {
        self.giver.clone()
    }
    pub fn get_receiver(&self) -> Person {
        self.receiver.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    group: Option<conf::Group>,
    name: String,
}

impl Person {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_group(&self) -> Option<conf::Group> {
        self.group.clone()
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



    pub fn assign_kks(users: &Vec<String>) -> Vec<KkPair> {
        let mut all = vec![];
        for name in users {
            all.push(parse_user(name.to_owned()));
        }
        perform_pairing(&all)
    }
}

/// Given a string this will construct a `Person` with the appropriate group if provided.
// TODO This function will not be needed once get proper configuration file
fn parse_user(user: String) -> Person {
    println!("{:?}", user);
    if user.contains(':') {
        let mut split = user.splitn(2, ':').map(|x| x.to_owned());
        let name: String = split.next().unwrap();
        let group: u32 = split.next()
            .unwrap()
            .parse()
            .unwrap();
        Person {
            name: name,
            group: Some(conf::Group {
                            id: group,
                            email: String::new(),
                        }),
        }
    } else {
        Person {
            name: user,
            group: None,
        }
    }
}

/// Performs the pairing of each giver to the receiver
fn perform_pairing(all: &Vec<Person>) -> Vec<KkPair> {
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
        if pair.giver.name.eq(&pair.receiver.name) && pair.giver.group.is_some() &&
           pair.giver.group.is_some() {
            // let giver_group = pair.giver
            //     .group
            //     .unwrap()
            //     .get_id();
            // let recvr_group = pair.receiver
            //     .group
            //     .unwrap()
            //     .get_id();
            // FIXME Just crappy until it has been cleaned up
            let giver_group = 0;
            let recvr_group = 1;
            if giver_group == recvr_group {
                return true;
            }
        }
    }
    false
}

/// Given the name of the giver will find the name of the receiver
pub fn find_kk(pairs: &Vec<KkPair>, needle: &String) -> Option<String> {
    for pair in pairs {
        if pair.giver.name.eq(needle) {
            return Some(pair.receiver.name.clone());
        }
    }
    None
}