use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Error;


fn main() {
    compile_bytecode();
    let bytecode = read_bytecode().unwrap();
    println!("{:?}", bytecode)
}

fn compile_bytecode() {
    Command::new("python").arg("-m").arg("main.py").output().expect("fail");
}


fn read_bytecode() -> Result<Vec<u8>, Error> {
    let mut file = File::open("main.pyc")?;

    let mut meta = vec![0, 8];
    file.read_exact(&mut meta);
    // println!("{:?}", meta);

    let mut bytecode = Vec::new();
    file.read_to_end(&mut bytecode)?;

    Ok(bytecode)
}


#[cfg(test)]
mod tests {
    use super::read_bytecode;

    // #[test]
    // fn test_bytes() {
    //     assert_eq!(read_bytecode())
    // }
}
