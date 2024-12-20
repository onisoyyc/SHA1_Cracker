// read command line args
use std::env::{
    //error handling
    env,
    error::Error,
    // read file
    fs::File,
    io::{BufRead, BufReader},
};
 const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error >>{
    let args: Vec<Strings> = env::args().collect(); // resizable array for string objects collected in the cli

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: </path/to/wordlist.txt> <sha1_hash>"); // build to allow user to input path to a file of hashes
        println!("Example: cargo run -- rockyou.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53"); //prints Hello, World!
        return;
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    // read wordlist from specified path, 2nd argument
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return OK(());
        }
        // println!("{}", line);
    }

    println!("password not found: {}", &common_password);

    Ok(())
}