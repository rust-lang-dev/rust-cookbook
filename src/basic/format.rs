//  all format in this example can be used in println! macro
pub fn test() {
    let best = format!(
        "The best programming language in the world is {}, no doubt",
        "PHP"
    );
    println!("{}", best);
    let hello = "hello";
    let world = "world";
    let hello_world = format!("{}, {}", world, hello);
    println!("{}", hello_world);

    let str_num = format!("String concatenate number {}", 11);
    println!("{}", str_num);

    let good_good_study = format!("{good} {good} {study}", good = "good", study = "study");
    println!("{}", good_good_study);
    let day_day_up = format!("{0} {0} {1}", "day", "up");
    println!("{}", day_day_up);
}
