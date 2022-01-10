mod front_of_house;

mod back_of_house;

fn serve_order() {}

pub fn paths_demo() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn making_structs_public() {
    // Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what toast we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn making_enums_public() {
    // If we make an enum public, all of its variants are then public. We only need the pub before the enum keyword
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// We can bring the `serving` module into scope with the `use` keyword
use crate::front_of_house::serving;

pub fn bringing_paths_into_scope() {
    serving::take_order();
    serving::take_payment();
}

// You can also bring an item into scope with use and a relative path
use self::front_of_house::hosting;

pub fn bringing_paths_into_scope_relative() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// The idiomatic way is to bring the function into scope directly, saving us from specifying the parent module
use crate::front_of_house::hosting::add_to_waitlist;

pub fn creating_idiomatic_use_paths() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
