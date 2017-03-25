use super::*;

use std::path::Path;


#[derive(Debug, Deserialize)]
pub struct KkConf {
    groups: Vec<Group>,
    participants: Vec<Participants>,
}

impl KkConf {
    pub fn build<P: AsRef<Path>>(path: P) -> KkConf {
        toml::from_str(&file_utils::read_from_file(path)).unwrap()
    }

    /// Returns the participants read from the configuration
    pub fn get_participants(&self) -> Vec<Participants> {
        self.participants.clone()
    }

    /// Returns the groups read from the configuration
    pub fn get_groups(&self) -> Vec<Group> {
        self.groups.clone()
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Participants {
    name: String,
    group: u32,
}

impl Participants {
    pub fn new(name: String, group: u32) -> Participants {
        Participants {
            name: name,
            group: group,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_group(&self) -> u32 {
        self.group
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    pub id: u32,
    pub email: String,
}
impl Group {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}