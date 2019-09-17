pub fn test() {
    println!("Move:");
    move_str();
    println!("Clone:");
    clone_str();
    println!("Push:");
    push_str();
}

// move into another
fn move_str() {
    let hello = "hello".to_string();
    // world is &str
    let world = "world";
    let hello_world = hello + world;
    println!("{}", hello_world);
}
fn clone_str() {
    // hello is &str
    let hello = "hello";
    let hello_clone = hello.clone();
    println!("clone: {}", hello_clone);
    println!("Original {} is still there", hello);
    println!("Original {} is still there", hello);
}
fn push_str() {
    let mut hello = "hello".to_string();
    let world = "world";
    hello.push_str(world);
    println!("{}", hello);
    println!("{}", hello);
}
