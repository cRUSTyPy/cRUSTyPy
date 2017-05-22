use std::process::Command;

fn main() {
    Command::new("python").arg("-m").arg("tacos.py").output().expect("fail");
}
