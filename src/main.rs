extern crate cursive;

use cursive::Cursive;
use cursive::views::Dialog;

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Dialog::text("...").title("I'm speechless")
    .button("Quit", |s| s.add_layer(Dialog::info("You Can't Quit. This is your life!"))));

    siv.run();
}
