extern crate rand;

use std::io::{BufRead, BufReader, Read};
use std::iter::FromIterator;
use std::fs::File;
use std::path::Path;

use rand::Rng;

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
    group: Option<u32>,
    name: String,
}

impl Person {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_group(&self) -> Option<u32> {
        self.group.clone()
    }
}

pub fn assign_kks_from_file<P: AsRef<Path>>(path: P) -> Vec<KkPair> {
    let mut all: Vec<Person> = Vec::new();
    let input = File::open(path).unwrap();
    let content = BufReader::new(input);
    for user in content.lines() {
        let un_user = user.unwrap();
        all.push(parse_user(un_user));
    }

    perform_pairing(&all)
}
pub fn assign_kks(users: &Vec<String>) -> Vec<KkPair> {
    let mut all = vec![];
    for name in users {
        all.push(parse_user(name.to_owned()));
    }
    perform_pairing(&all)
}


/// Given a string this will construct a `Person` with the appropriate group if provided.
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
            group: Some(group),
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
            let giver_group = pair.giver.group.unwrap();
            let recvr_group = pair.receiver.group.unwrap();
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