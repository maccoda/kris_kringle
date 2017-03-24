extern crate rand;

use std::io::{self, Write, Read, BufReader, BufRead};
use std::fs::File;
use rand::Rng;

struct KkPair {
    giver: Person,
    receiver: Person,
}

#[derive(Debug, Clone)]
struct Person {
    group: Option<u32>,
    name: String,
}

fn main() {
    let mut all: Vec<Person> = Vec::new();
    // HACK These should be command line parameters
    let stdin = false;
    let grouping = true;

    if stdin {
        // Obtain from stdin
        println!("How many people are part of this KK group?");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed read");
        let num = num.trim().parse().expect("Failed parse");
        for _ in 0..num {
            println!("Give me a name: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed read");
            all.push(Person {
                name: name,
                group: None,
            });
        }
    } else {
        // Obtain from file
        println!("Reading from file");
        // FIXME This is crappy hardcoding for lack of testing
        let input = File::open("tests/resources/groups.txt").unwrap();
        let content = BufReader::new(input);
        for user in content.lines() {
            println!("{:?}", user);
            let un_user = user.unwrap();
            if grouping {
                let mut split = un_user.splitn(2, ":").map(|x| x.to_owned());
                let name: String = split.next().unwrap();
                let group: u32 = split.next().unwrap().parse().unwrap();
                // TODO will need a new algorithm or tweaked algorithm to not
                // allow people to have another person that is in the same group
                all.push(Person {
                    name: name,
                    group: Some(group),
                });
            } else {
                all.push(Person {
                    name: un_user,
                    group: None,
                });
            }
        }
    }

    // Construct the pairing
    let mut pairs: Vec<KkPair> = Vec::new();
    for person in &all {
        pairs.push(KkPair {
            giver: person.clone(),
            receiver: person.clone(),
        });
    }
    while invalid_map(&pairs) {
        let giver1_index = rand::thread_rng().gen_range(0, all.len());
        let giver2_index = rand::thread_rng().gen_range(0, all.len());
        let temp = pairs[giver1_index].receiver.clone();
        pairs[giver1_index].receiver = pairs[giver2_index].receiver.clone();
        pairs[giver2_index].receiver = temp;
    }

    // Create the output for the results
    for pair in &pairs {
        let mut file_name: String = pair.giver.name.to_owned();
        file_name.push_str(".kk");
        let mut file = File::create(file_name).unwrap();
        let content: &[u8] = &(pair.receiver.name.to_owned().into_bytes())[..];
        file.write_all(content).unwrap();
    }

    // Time to ask who is doing this
    loop {
        println!("Who are you exactly???");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("No name provided");
        let name: String = name.trim().to_string();
        match find_kk(&pairs, &name) {
            Some(rec) => println!("Give to {}", rec),
            None => println!("You are not welcome here {}!!!", name),
        }
        println!("Got it?");
        let mut name = String::new();
        // Don't actually care what they write it is clearing
        io::stdin().read_line(&mut name).expect("No name provided");
        // Clears the terminal screen
        print!("{}[2J", 27 as char);
    }

}

/// Determines if this is a valid match up. Initially needs to check that
/// user is not giving present to themselves. Second check, if enabled is to
/// confirm that groups are different.
fn invalid_map(pairs: &Vec<KkPair>) -> bool {
    for pair in pairs {
        println!("{:?} --> {:?}", pair.giver, pair.receiver);
        if pair.giver.name.eq(&pair.receiver.name) {
            // Check if both have a group
            if pair.giver.group.is_some() && pair.giver.group.is_some() {
                let giver_group = pair.giver.group.unwrap();
                let recvr_group = pair.receiver.group.unwrap();
                if giver_group == recvr_group {
                    return true;
                }
            }
        }
    }
    false
}

/// Given the name of the giver will find the name of the receiver
fn find_kk(pairs: &Vec<KkPair>, needle: &String) -> Option<String> {
    for pair in pairs {
        if pair.giver.name.eq(needle) {
            return Some(pair.receiver.name.clone());
        }
    }
    None
}
