use monkey_typewriter::create_slug;
use std::env;

fn main() {
    println!("{}", String::from("Hello"));
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let slug_length = &args
        .get(1)
        .unwrap_or(&"2".to_owned())
        .parse::<u32>()
        .unwrap_or(2);
    println!("Generating slug length = {}", slug_length);
    let slug = create_slug(*slug_length);
    println!("Generated = {}", slug);
}
