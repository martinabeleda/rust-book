fn main() {
    use std::collections::HashMap;

    // Creating a mutable HashMap inserting values
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Creating with insert: {:?}", scores);

    // Contructing using iterators and collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Creating with iterators and collect: {:?}", scores);

    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Accessing values: {:?}", score);

    // Iterating over key, value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("Overwriting: {:?}", scores);

    // Only inserting if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("Or insert: {:?}", scores);

    // Updating a value based on the old value
    // Implment a word counter
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // `or_insert` returns a mutable reference (&mut V) to the value for this key.
        // In order to assign to count, we must dereference
        *count += 1;
    }
    println!("Word counter: {:?}", map)
}
