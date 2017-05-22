use std::process::Command;

fn main() {
    Command::new("python").arg("-m").arg("main.py").output().expect("fail");
}
