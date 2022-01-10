fn main() {
    // Creating a new empty string
    let mut _s = String::new();

    // Creating a string from a literal
    let data = "initial contents";
    let s = data.to_string();
    println!("From literal: {}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("From literal directly: {}", s);

    // Can also use String::from
    let s = String::from("initial contents");
    println!("Using String::from: {}", s);

    // Strings are also UTF-8 encoded so can include any properly encoded data
    let s = String::from("Hello 👋");
    println!("Can use any UTF-8 encoded data: {}", s);

    // Appending to a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Appending: {}", s);

    // Adding strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Adding strings: {}", s);

    // the format! macro uses references so does not take ownership of any of it's parameters
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatting strings: {}", s);

    // Slicing over  strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("Slice: {}", s);

    // Iterating over chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Iterating over bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
