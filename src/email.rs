use std::env;

// use lettre::email::EmailBuilder;
// use lettre::transport::EmailTransport;
// use lettre::transport::smtp;

/// This function would be the final step for the process as it is the method in which
/// we will inform everyone of the allocations. This will use the information provided in
/// the configuration to determine which email address to use for each giver. It will send a
/// single email for each group containing details for each giver that is a part of that group.
pub fn send_emails(conf: &super::KrisKringles) -> Result<(), String> {
    debug!("Time to start sending emails");
    info!("Using the email: {:?}", env::var("GMAIL_USERNAME").unwrap());
    // let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", smtp::SUBMISSION_PORT))
    //     .expect("failed to create transport")
    //     .credentials(&env::var("GMAIL_USERNAME").unwrap(),
    //                  &env::var("GMAIL_PASSWORD").unwrap())
    //     .build();
    // for group in &conf.configuration.get_groups() {
    //     info!("Constructing email for {:?}", group.get_email());

    //     let body = collate_group_content(group, conf);

    //     let email = EmailBuilder::new()
    //         .from("santa.claus@northpole.com")
    //         .to(group.get_email().as_ref())
    //         .subject("Kris Kringle allocation")
    //         .body(body.as_ref())
    //         .build()
    //         .expect("Someone has stuffed up here..");


    //     if transport.send(email).is_err() {
    //         return Err(format!("Unable to send email to {:?}", group.get_email()));
    //     }
    // }


    Ok(())
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
