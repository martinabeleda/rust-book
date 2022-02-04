use gui::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// Implementing the draw trait for one of the user's own
// structs
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw the SelectBox
    }
}

fn main() {
    let screen = Screen {
        components = vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("hello"),
            }),
        ]
    };

    screen.run();
}
