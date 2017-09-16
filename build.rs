use std::process::Command;

fn main() {
    Command::new("./install_npm.sh").spawn().unwrap();


    Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir("client")
        .spawn()
        .unwrap();
}
