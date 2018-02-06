extern crate cursive;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate nix;

use simplelog::*;

use cursive::Cursive;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::TextView;
use cursive::traits::Identifiable;
use cursive::traits::Boxable;

use nix::pty;

use std::fs::{File, OpenOptions};

use std::process::*;
use std::os::unix::io::FromRawFd;

mod shell;
mod views;

use views::term::TermView;

fn main() {
    let mut opts = OpenOptions::new();
    opts.append(true);
    opts.create_new(true);
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Warn, Config::default(), opts.open("cue.errors.log").unwrap()),                        
            WriteLogger::new(LevelFilter::Info, Config::default(), opts.open("cue.info.log").unwrap()),
            WriteLogger::new(LevelFilter::Trace, Config::default(), opts.open("cue.debug.log").unwrap()),
        ]
    ).unwrap();
    info!("Starting C.U.E. - Hello World");

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
    //WriteLogger::init(LevelFilter::Off, Config::default(), File::create("CUE.log").unwrap()).unwrap();
    
    let mut siv = Cursive::new();

    siv.set_fps(30);

    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    siv.add_layer(
        Dialog::new()
        .title("Command")
        //.padding((1, 1, 1, 0))
        .content(EditView::new()
            .on_submit(run_cmd)
            .with_id("name")
            .fixed_width(20)
            ),
    );

    //siv.add_layer(Dialog::text("...").title("I'm speechless")
    //.button("Quit", |s| s.add_layer(Dialog::info("You Can't Quit. This is your life!"))));

    siv.run();
}

fn run_cmd(s: &mut Cursive, cmd: &str) {
    let cmd: Vec<&str> = cmd.split_whitespace().collect();

    let p = pty::openpty(None, None).unwrap();
    let a = unsafe {
        Command::new(cmd[0])
            .args(&cmd[1..])
            .stdout(Stdio::from_raw_fd(p.slave))
            .stderr(Stdio::from_raw_fd(p.slave))
            .stdin(Stdio::piped())
            .spawn()
    };

    let a = a.unwrap();

    let outfile = unsafe { File::from_raw_fd(p.master) };
    let infile = outfile.try_clone().expect("file didn't clone");

    let t = TermView::new(80, 40, outfile, infile);

    s.add_layer(Dialog::around(t).button("Quit", |s| s.quit()));

    // let mut buf: String = String::new();
    // //let mut out = out.bytes();
    // // loop {
    // //     let c = out.next().unwrap().unwrap();
    // //     io::stdout().write(&[c]);
    // // }
    // out.read_to_string(&mut buf);
    // show_term(s, &buf);
}

// fn show_term(s: &mut Cursive, data: &str) {
//     let mut t = TermView::new(80,40);
//     t.put_str(data);
//     s.add_layer(Dialog::around(t).button("Quit", |s| s.quit()));

// }

fn show_popup(s: &mut Cursive, data: &str) {
    s.pop_layer();
    s.add_layer(Dialog::around(TextView::new(data)).button("Quit", |s| s.quit()));
}
