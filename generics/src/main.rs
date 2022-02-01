fn main() {
    // In struct definitions
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Integer point: {:?}", integer);
    println!("Float point: {:?}", float);

    // Generics with multiple defined types
    #[derive(Debug)]
    struct MultiTypePoint<T, U> {
        x: T,
        y: U,
    }
    
    let both_integer = MultiTypePoint { x: 5, y: 10 };
    let both_float = MultiTypePoint { x: 1.0, y: 4.0 };
    let integer_and_float = MultiTypePoint { x: 5, y: 4.0 };
    println!("Both integer point: {:?}", both_integer);
    println!("Both float point: {:?}", both_float);
    println!("Integer and float point: {:?}", integer_and_float);

    // In method definitions
    // Implement a method which returns a reference to the data in `x`
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
