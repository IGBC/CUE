extern crate cursive;

use cursive::{Printer, XY, view};
use cursive::event::*;
use cursive::vec::Vec2;

use std::{thread, time};
use std::fs::File;
use std::io::{self, Write,Bytes};
use std::io::prelude::*;
use std::sync::{Mutex, Arc};

enum TermViewState {
    Printing, // Currently printing out normally
    ESC, // Recieved ESC 
    CSI, // Recieved ESC[ buffering command code
}

struct TermViewData {
    buffer: Box<[char]>, // Screen Buffer sized width * height
    size: XY<usize>, // Size of window
    cursor: XY<usize>, // Current postion of the input cursor
    rx: Bytes<File>, // File handle to read from
    tx: File, // File handle to write to

    state: TermViewState,
    cmd_string: String,
}

pub struct TermView {
    c: Arc<Mutex<TermViewData>> // Mutable data container
}

impl TermView {
    /// Creates a new TermView with the given content.
    pub fn new(w: usize, h: usize, rx: File, tx: File) -> Self {
        // Create inner container that contains all mutable data
        let v = TermViewData {
            size: XY::new(w,h),
            buffer: vec![' '; w*h].into_boxed_slice(),
            cursor: XY::new(0,0),
            rx: rx.bytes(),
            tx: tx,
            state: TermViewState::Printing,
            cmd_string: String::from(""),
        };

        //Atomic Reference Counter wrapping a mutex lets two threads share this data
        let arc = Arc::new(Mutex::new(v));

        //wrapping the ARC in a TermView lets this look like a normal widget to cursive 
        let term_view = TermView {
            c: Arc::clone(&arc),
        };

        // Create IO thread allows updating the buffer without blocking the main thread
        thread::spawn(move || { Self::io_thread(Arc::clone(&arc)); });
        term_view
    }

    // Infinate loop that is always updating the buffer.
    fn io_thread(data: Arc<Mutex<TermViewData>>) {
        loop { // Do forever
            let mut t = match data.lock() { // WRONG
                Ok(cont) => cont,
                Err(_) => continue, // lock() is a blocking call. this is a different error
            };
            t.read_char(); // this is probably blocking.
            drop(t); // mutex is cleared here
            //sleep one second
            // thread::sleep(time::Duration::new(1,0)); // DEBUG
        }
    }
}

impl TermViewData {
    /// Moves cursor to given coordinates
    /// decends at the end of the line
    /// stalls at the end of the screen (no scrolling)
    fn move_cursor(&mut self, x: usize, y: usize) {
        let w = self.size.x; 
        let h = self.size.y;
        let mut x = x;
        let mut y = y; 
        if !(x < w) {
            x = 0;
            y = y+1;
        }

        if !(y < h) {
            x = w - 1;
            y = h - 1;
        }

        self.cursor.x = x;
        self.cursor.y = y;
    }

    /// Returns the character at the given X and Y coodinates
    fn get_char(&self, x: usize, y: usize) -> char {
        self.buffer[(y*self.size.x)+x]
        
        // match self.buffer[(y*self.size.x)+x] {
        //     '\x00'...'\x0F' => 'c',
        //     '\x20'          => ' ',
        //     '\x21'...'\x7E' => 'p',
        //     '\x7F'          => 'd',
        //     _ => 'U',
        // }
    }

    fn handle_csi(&mut self, final_char: char) {
        //Get private flag
        let mut first_char =  match self.cmd_string.chars().next() {
            Some('?') => "?",
            _ => "",
        };

        // if first character exists strip it from the string.
        let cmd = match first_char {
            "" => &self.cmd_string,
            _ => &self.cmd_string[1..],
        };        

        // Split Parameters
        let args: Vec<&str> = cmd.split(|c| c == ';' || c == ':').collect();
        
        //Cast what's left into integers
        let mut i_args: Vec<Option<i32>> = Vec::new();
        for x in args {
            if x == "" {
                i_args.push(None);
            } else {
                let f = match x.parse() {
                    Ok(i) => {
                        let t: i32 = i;
                        Some(t)
                    },
                    Err(_) => None,
                };
                i_args.push(f);
                
            }
        }
        
        // Match on Command Character
        match (first_char, i_args.len(), final_char) {
            _ => eprintln!("Unknown command code: {}[{}]{} from string {}{}", first_char, i_args.len(), final_char, &self.cmd_string, final_char),
        }
    }

    /// Print given character at the cursor.
    fn put_char(&mut self, c: char) {
        let x = self.cursor.x;
        let y = self.cursor.y;
        // matching corner cases here
        match self.state {
            TermViewState::Printing => { 
                match c {
                    '\x00' => self.move_cursor(x+1, y), // NULL interpet as space
                    '\x01'...'\x06' => (), // nonprinting, ignore
                    '\x07' => (), // TODO: BELL
                    '\x08' => self.move_cursor(x-1, y), // backspace
                    '\x09' => (), // TODO: Tabs
                    '\x0A' => self.move_cursor(0, y+1), // newline 
                    '\x10' => (), // TODO: Find out what a vertical tab is
                    '\x0C' => (), // nonprinting, ignore
                    '\x0D' => self.move_cursor(0, y), // carriage return
                    '\x0E'...'\x1A' => (), // nonprinting, ignore
                    '\x1B' => self.state = TermViewState::ESC, //Jump ESC
                    '\x0C'...'\x1F' => (), // nonprinting, ignore
                    '\x7F' => (), // DEL, ignore
                    _ => { // printing ascii and Unicode
                        self.buffer[(y*self.size.x)+x] = c;
                        self.move_cursor(x+1, y);
                    }, 
                };
            },

            TermViewState::ESC => {
                match c {
                    //ESC
                    '\x1B' => (), //Stay in escape mode
                    // [
                    '\x5B' => {
                        self.cmd_string = String::from("");
                        self.state = TermViewState::CSI;
                    },
                    // ]
                    // TODO OSI

                    _ => { // Return to printing and then output char
                        self.state = TermViewState::Printing;
                        self.put_char(c);
                    },
                };
            },

            TermViewState::CSI => {
                match c {
                    '\x00'...'\x1F' => (), // TODO break C0 into a function
                    
                    '\x30'...'\x3F' => self.cmd_string.push(c), //Parameter Bytes - add too string
                    '\x40'...'\x7E' => {
                        self.handle_csi(c); // CALL
                        self.cmd_string = String::from("");
                        self.state = TermViewState::Printing; //RESET
                    },

                    _ => (), // Ignore everything else. 
                };
            },
        }
    }

    /// Helper function to print an entire slice at once.
    pub fn put_str(&mut self, s: &str) {
        for c in s.chars() { // iterate
            self.put_char(c); // print
        }
    }

    /// Read next byte from rx file handle
    fn read_char(&mut self) {
        let c = self.rx.next(); // Get next char from input
        // is next a blocking call?
        // Unwrap and skip errors
        let c = match c { // WRONG
            Some(Ok(r)) => r,
            Some(Err(_)) => return, // This indicates a more serious IO error. 
            None => return,
        };

        // cast to character and print.
        //io::stderr().write(&[c]);
        self.put_char(c as char);
    }

    /// The renderer, let the TermView Call this.
    fn draw(&self, printer: &Printer) {
        let w = self.size.x;
        let h = self.size.y;
        // Itterate over the screen x,y
        for x in 0..w {
            for y in 0..h {
                // print the character at the current x,y
                printer.print((x,y), &self.get_char(x, y).to_string());
            }
        }
        // Print Debug information on one extra line.
        printer.print((0,h+1), &format!("len {}", self.buffer.len()))
    }

    fn get_size(&mut self) -> Vec2 {
        // +2 here adds an extra 2 lines for debug info at the bottom of the term.
        Vec2::new(self.size.x, self.size.y+2)
    }
}

// The implementation is pretty much stubbed out
// and redirected to functions inside of the TermViewData 
// where the fields are safely mutexed.
impl view::View for TermView {
    fn draw(&self, printer: &Printer) {
        let t = self.c.lock().unwrap();
        t.draw(printer);
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            _ => return EventResult::Ignored,
        }
        EventResult::Consumed(None)
    }

    #[allow(unused_variables)]
    fn required_size(&mut self, size: Vec2) -> Vec2 {
        let mut t = self.c.lock().unwrap();
        t.get_size()
    }
}