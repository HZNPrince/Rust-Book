mod front_of_house;

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizers{
        Soup,
        Starters(String),
    }
}

fn deliver_order(){}

use front_of_house::hosting;

pub fn eat_at_restaurent(){
    // Let order the breakfast with French Loaf
    let mut prince_meal = back_of_house::Breakfast::summer("French Loaf");
    // Ahh fk I want pita break
    prince_meal.toast = "Pita Bread".to_string();
    println!("I'd like to have {} please.", prince_meal.toast);

    // Now lets order something for lunch
    let lunch_order1 = crate::back_of_house::Appetizers::Soup;
    let lunch_order2 = back_of_house::Appetizers::Starters("Manchurian".to_string());
}
