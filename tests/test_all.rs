extern crate kris_kringle;

use kris_kringle::KrisKringles;

#[test]
fn correct_allocation_files() {
    let kks = KrisKringles::build_kks_from_file("tests/resources/full.toml");
    kks.write_kks_to_file("results.txt");
    check_correct_allocation(&kks, "Dylan", vec!["Jordan", "Luke"]);
    check_correct_allocation(&kks, "Jordan", vec!["Dylan", "Luke"]);
    check_correct_allocation(&kks, "Luke", vec!["Dylan", "Jordan"]);
    check_correct_allocation(&kks, "Olivia", vec!["Alec", "Dean"]);
    check_correct_allocation(&kks, "Alec", vec!["Olivia", "Dean"]);
    check_correct_allocation(&kks, "Dean", vec!["Olivia", "Alec"]);
    check_correct_allocation(&kks, "Alessia", vec!["Sienna"]);
    check_correct_allocation(&kks, "Sienna", vec!["Alessia"]);
    check_correct_allocation(&kks, "Isabella", vec!["Max", "Luca"]);
    check_correct_allocation(&kks, "Max", vec!["Isabella", "Luca"]);
    check_correct_allocation(&kks, "Luca", vec!["Isabella", "Max"]);
}

fn check_correct_allocation(kks: &KrisKringles,
                            giver_name: &'static str,
                            group_members: Vec<&'static str>) {
    println!("Checking for {:?}", giver_name);
    let recv = kks.find_kk(giver_name);
    assert!(recv.is_some());
    let recv = recv.unwrap();
    assert!(!recv.eq(giver_name));
    for pp in group_members {
        assert!(!recv.eq(pp));
    }
    // Check the file was created
    assert!(kris_kringle::file_utils::check_file_exists(format!("{}.kk", giver_name)));
}
