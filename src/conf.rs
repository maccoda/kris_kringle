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

    pub fn new(participants: Vec<Participants>) -> KkConf {
        KkConf {
            groups: vec![],
            participants
        }
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
    pub email: Option<String>,
}
impl Group {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_email(&self) -> Option<String> {
        self.email.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build() {
        let result = super::conf::KkConf::build("tests/resources/full.toml");

        let groups = result.get_groups();
        assert_eq!(1, groups[0].get_id());
        assert!(groups[0].get_email().unwrap().eq("test1@hotmail.com"));
        assert_eq!(2, groups[1].get_id());
        assert!(groups[1].get_email().unwrap().eq("test2@gmail.com"));
        assert_eq!(3, groups[2].get_id());
        assert!(groups[2].get_email().unwrap().eq("test3@yahoo.com"));
        assert_eq!(4, groups[3].get_id());
        assert!(groups[3].get_email().unwrap().eq("test4@outlook.com"));

        let participants = result.get_participants();
        assert_eq!(1, participants[0].get_group());
        assert!(participants[0].get_name().eq("Dylan"));
        assert_eq!(1, participants[1].get_group());
        assert!(participants[1].get_name().eq("Jordan"));
        assert_eq!(1, participants[2].get_group());
        assert!(participants[2].get_name().eq("Luke"));
        assert_eq!(2, participants[3].get_group());
        assert!(participants[3].get_name().eq("Olivia"));
        assert_eq!(2, participants[4].get_group());
        assert!(participants[4].get_name().eq("Alec"));
        assert_eq!(2, participants[5].get_group());
        assert!(participants[5].get_name().eq("Dean"));
        assert_eq!(3, participants[6].get_group());
        assert!(participants[6].get_name().eq("Alessia"));
        assert_eq!(3, participants[7].get_group());
        assert!(participants[7].get_name().eq("Sienna"));
        assert_eq!(4, participants[8].get_group());
        assert!(participants[8].get_name().eq("Isabella"));
        assert_eq!(4, participants[9].get_group());
        assert!(participants[9].get_name().eq("Max"));
        assert_eq!(4, participants[10].get_group());
        assert!(participants[10].get_name().eq("Luca"));

    }
}
