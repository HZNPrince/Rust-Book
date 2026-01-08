use trait_objects::{self, Button, Draw, Screen, SelectBox};
fn main() {
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(SelectBox {
            width: 10,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("No"),
                String::from("Maybe"),
            ],
        }),
        Box::new(Button {
            width: 5,
            height: 5,
            label: String::from("Ok"),
        }),
    ];
    let screen = Screen { components };

    screen.run();
}
