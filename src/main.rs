extern crate cursive;

use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::TextView;
use cursive::traits::Identifiable;
use cursive::traits::Boxable;
use std::io::Read;

mod shell;
use shell::Shell;

fn main() {
    let mut siv = Cursive::new();
    
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    siv.add_layer(Dialog::new()
        .title("Command")
        //.padding((1, 1, 1, 0))
        .content(EditView::new()
            .on_submit(show_popup)
            .with_id("name")
            .fixed_width(20)
            )
        );
            
    //siv.add_layer(Dialog::text("...").title("I'm speechless")
    //.button("Quit", |s| s.add_layer(Dialog::info("You Can't Quit. This is your life!"))));

    siv.run();
}

fn show_popup(s: &mut Cursive, name: &str) {
    if name.is_empty() {
        s.add_layer(Dialog::info("No Comment!"));
    } else {
        let mut shell = Shell::new();
        let c = shell.exec(name, &[]);
        let mut out = c.stdout.unwrap();
        let mut buf: String = String::new();
        out.read_to_string(&mut buf);
        
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(buf))
            .button("Quit", |s| s.quit()));
    }
}
