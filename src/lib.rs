extern crate lettre;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;

use rand::Rng;

mod conf;
mod email;
pub mod file_utils;

#[derive(Debug)]
struct KkPair {
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
    /// Construct the kris kringle matcher from the provided configuration file. The file will need to be
    /// a TOML file following the structure seen in the file at `tests/resources/full.toml`
    pub fn build_kks_from_file<P: AsRef<Path>>(path: P) -> KrisKringles {
        let conf = conf::KkConf::build(path);
        let pairs = perform_pairing(&conf.get_participants());

        KrisKringles {
            configuration: conf,
            pairs: pairs,
        }

    }

    /// Once the configuration has been loaded, this will allow for the allocation of the kris kringles following
    /// the criteria that a pair must consist of two separate people (i.e. cannot give a present to themself) and
    /// they must be from a separate group as allocated in the configuration.
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

    /// Returns all participants used in the Kris Kringle allocation
    pub fn get_participants(&self) -> Vec<String> {
        self.configuration
            .get_participants()
            .iter()
            .map(|x| x.get_name())
            .collect()
    }

    /// Sends emails to the allocated giver of the Kris Kringle pair. This function
    /// will fail if the allocation has not yet been performed.
    // TODO Add some error handling
    pub fn email_givers(&self) -> Result<bool, String> {
        if invalid_map(&self.pairs) {
            return Err(String::from("The pairs have not yet been allocated!!!"));
        }

        email::send_emails(self).unwrap();
        Ok(true)
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
    // for pair in &pairs {
    //     println!("{:?} --> {:?}", pair.giver, pair.receiver);
    // }
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
        // TODO Change all the below to logs
        // println!("Comparing {:?} - {:?}", pair.giver, pair.receiver);

        if pair.giver.get_name().eq(&pair.receiver.get_name()) {
            // println!("It is invalid");
            return true;
        }
        let giver_group = pair.giver.get_group();
        let recvr_group = pair.receiver.get_group();
        if giver_group == recvr_group {
            // println!("It is invalid");
            return true;
        }
    }
    // println!("It is valid");
    false
}
