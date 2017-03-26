use std::env;

use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp;

pub fn send_emails(conf: &super::KrisKringles) -> Result<bool, String> {
    for group in &conf.configuration.get_groups() {

        collate_group_content(group, conf);

        let email = EmailBuilder::new()
            .from("santa.claus@northpole.com")
            .to("test")
            .subject("Kris Kringle allocation")
            .body("Your KK is...")
            .build()
            .expect("Someone has stuffed up here..");

        let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com",
                                                             smtp::SUBMISSION_PORT))
                .expect("failed to create transport")
                .credentials(&env::var("GMAIL_USERNAME").unwrap(),
                             &env::var("GMAIL_PASSWORD").unwrap())
                .build();

        transport.send(email).unwrap();
    }


    Ok(true)
}

/// Collates all of the details for a single group into a single string as the emails are to be sent, 1 per group.
fn collate_group_content(group: &super::conf::Group, all_pairs: &super::KrisKringles) -> String {
    let names_in_group: Vec<String> = all_pairs.configuration
        .get_participants()
        .iter()
        .filter(|x| x.get_group() == group.get_id())
        .map(|x| x.get_name())
        .collect();

    let mut result = String::new();
    for person in names_in_group {
        let recv = all_pairs.find_kk(&person).expect(&format!("Unable to find giver {:?}", person));
        result.push_str(&build_content(&person, &recv));
    }

    result
}

/// Builds content for a single giver receiver pair. Produces 'giver --> receiver'.
fn build_content(giver: &str, receiver: &str) -> String {
    let mut result = String::new();
    result.push_str(giver);
    result.push_str(" --> ");
    result.push_str(receiver);
    result.push_str("\n");
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_content() {
        let actual = super::build_content("Alice", "Bob");
        assert_eq!(actual, "Alice --> Bob\n");
    }

    // TODO Add more tests. Will be a bit painful as need to manually build.
}
