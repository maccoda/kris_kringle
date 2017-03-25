use super::*;

use std::path::Path;


#[derive(Debug)]
pub struct KkConf {
    groups: Vec<Group>,
    participants: Vec<super::Person>,
}

impl KkConf {
    pub fn build<P: AsRef<Path>>(path: P) -> KkConf {
        toml::from_str(&file_utils::read_from_file(path));
    }

    /// Returns the participants read from the configuration
    pub fn get_participants(&self) -> Vec<super::Person> {
        self.participants.clone()
    }

    /// Returns the groups read from the configuration
    pub fn get_groups(&self) -> Vec<Group> {
        self.groups.clone()
    }
}





#[derive(Debug, Clone)]
pub struct Group {
    pub id: u32,
    pub email: String,
}
impl Group {
    pub fn get_id(&self) -> u32 {
        self.id
    }
}