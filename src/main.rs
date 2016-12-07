extern crate rand;

use std::io;
use rand::Rng;

struct KkPair {
    giver: String,
    receiver: String,
}

fn main() {
    let mut all = Vec::new();
    println!("How many people are part of this KK group?");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed read");
    let num = num.trim().parse().expect("Failed parse");
    for _ in 0..num {
        println!("Give me a name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed read");
        all.push(name);
    }

    let mut pairs: Vec<KkPair> = Vec::new();
    for person in &all {
        pairs.push(KkPair {
            giver: (*person).trim().to_string(),
            receiver: (*person).trim().to_string(),
        });
    }
    while invalid_map(&pairs) {
        let giver1_index = rand::thread_rng().gen_range(0, all.len());
        let giver2_index = rand::thread_rng().gen_range(0, all.len());
        let temp = pairs[giver1_index].receiver.clone();
        pairs[giver1_index].receiver = pairs[giver2_index].receiver.clone();
        pairs[giver2_index].receiver = temp;
    }

    // for pair in &pairs {
    //     println!("{:?} -> {:?}", pair.giver, pair.receiver);
    // }

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

fn invalid_map(pairs: &Vec<KkPair>) -> bool {
    for pair in pairs {
        if pair.giver.eq(&pair.receiver) {
            return true;
        }
    }
    false
}

fn find_kk(pairs: &Vec<KkPair>, needle: &String) -> Option<String> {
    for pair in pairs {
        if pair.giver.eq(needle) {
            return Some(pair.receiver.clone());
        }
    }
    None
}
