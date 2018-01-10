extern crate cursive;
extern crate nix;

use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::TextView;
use cursive::traits::Identifiable;
use cursive::traits::Boxable;

use nix::pty;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::process::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

mod shell;
mod term;

use term::TermView;
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
            .on_submit(run_cmd)
            .with_id("name")
            .fixed_width(20)
            )
        );
            
    //siv.add_layer(Dialog::text("...").title("I'm speechless")
    //.button("Quit", |s| s.add_layer(Dialog::info("You Can't Quit. This is your life!"))));

    siv.run();
}

fn run_cmd(s: &mut Cursive, cmd: &str) {
    let cmd: Vec<&str> = cmd.split_whitespace().collect();
    

    let p = pty::openpty(None,None).unwrap();
    let a = unsafe {
        Command::new(cmd[0]).args(&cmd[1..])
                 .stdout(Stdio::from_raw_fd(p.slave))
                 .stderr(Stdio::from_raw_fd(p.slave))
                 .stdin (Stdio::piped())
                 .spawn()
    };

    let a = a.unwrap();

    let mut out = unsafe { File::from_raw_fd(p.master) };
    let mut buf: String = String::new();
    //let mut out = out.bytes();
    // loop {
    //     let c = out.next().unwrap().unwrap();
    //     io::stdout().write(&[c]);        
    // } 
    out.read_to_string(&mut buf);
    show_term(s, &buf);
}

fn show_term(s: &mut Cursive, data: &str) {
    let mut t = TermView::new(80,40);
    t.put_str(data);
    s.add_layer(Dialog::around(t).button("Quit", |s| s.quit()));

}

fn show_popup(s: &mut Cursive, data: &str) {
    s.pop_layer();
    s.add_layer(Dialog::around(TextView::new(data)).button("Quit", |s| s.quit()));
}
