use std::ops::Deref;

fn main() {
    // Dereferencing a pointer to an i32
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> like a reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Defining your own smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
