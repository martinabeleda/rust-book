fn main() {
    // Create a new empty vector
    let v: Vec<i32> = Vec::new();
    println!("Creating a new vector: {:?}", v);

    // Let rust infer the type of the vector based on the elements inside
    let v = vec![1, 2, 3, 4];
    println!("Inferring the type of a vector: {:?}", v);

    // Updating a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Updating a vector: {:?}", v);

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5, 6];
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over the immutable values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Iterating over mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Mutating elements of a vector: {:?}", v);

    // Using an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Using enums in vectors: {:?}", row)
}
