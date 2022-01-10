fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
}

fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    // Because the public struct Breakfast contains a private variable `seasonal_fruit`, we need to provide a
    // public constructor since a user can't touch the `seasonal_fruit` field
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
