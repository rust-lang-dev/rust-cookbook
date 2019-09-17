use std::env;

pub fn test() {
    println!("Listing all env vars:");
    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }

    let path = "PATH";
    match env::var(path) {
        Ok(val) => println!("{}: {}", path, val),
        Err(e) => println!("Couldn't get env var {}: {}", path, e),
    }
}