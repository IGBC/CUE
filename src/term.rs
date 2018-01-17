extern crate cursive;

use cursive::Printer;
use cursive::XY;
use cursive::view::View;
use cursive::event::*;
use cursive::vec::Vec2;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::thread;
use std::sync::{Mutex, Arc};
use std::time;

struct TermViewData {
    buffer: Box<[char]>, // Screen Buffer sized width * height
    size: XY<usize>, // Size of window
    cursor: XY<usize>, // Current postion of the input cursor
    rx: File, // File handle to read from
    tx: File, // File handle to write to
}

pub struct TermView {
    c: Arc<Mutex<TermViewData>> // Mutable data container
}

impl TermView {
    /// Creates a new TermView with the given content.
    pub fn new(w: usize, h: usize, rx: File, tx: File) -> Self {
        let mut v = TermViewData {
            size: XY::new(w,h),
            buffer: vec![' '; w*h].into_boxed_slice(),
            cursor: XY::new(0,0),
            rx: rx,
            tx: tx,
        };

        let arc = Arc::new(Mutex::new(v));

        let mut term_view = TermView {
            c: Arc::clone(&arc),
        };
        // Create IO thread with reference to mutable data inside thingy.
        thread::spawn(move || { Self::io_thread(Arc::clone(&arc)); });
        term_view
    }

    fn io_thread(data: Arc<Mutex<TermViewData>>) {
        loop {
            {
                let mut t = match data.lock() {
                    Ok(cont) => cont,
                    Err(_) => continue,
                };
                t.read_char();
                drop(t);
            } // mutex is cleared here
            //sleep one second
            thread::sleep(time::Duration::new(1,0)); // DEBUG
        }
    }
}

impl TermViewData {
    pub fn move_cursor(&mut self, x: usize, y: usize) {
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

    fn get_char(&self, x: usize, y: usize) -> char {
        self.buffer[(y*self.size.x)+x]
    }

    pub fn put_char(&mut self, c: char) {
        let x = self.cursor.x;
        let y = self.cursor.y;
        match c {
            '\n' => self.move_cursor(0, y+1),
            _ => {
                self.buffer[(y*self.size.x)+x] = c;
                self.move_cursor(x+1, y);
            }  
        };
        
    }

    pub fn put_str(&mut self, s: &str) {
        for c in s.chars() {
            self.put_char(c);
        }
    }

    fn read_char(&mut self) {
        let mut f = self.rx.try_clone().expect("fuckit").bytes(); // WRONG
        let c = f.next();
        let c = match c {
            Some(Ok(r)) => r,
            Some(Err(_)) => return, // WRONG
            None => return,
        };
        self.put_char(c as char);
    }
}
impl View for TermViewData {
    fn draw(&self, printer: &Printer) {
        
        let w = self.size.x;
        let h = self.size.y;
        for x in 0..w {
            for y in 0..h {
                printer.print((x,y), &self.get_char(x, y).to_string());
            }
        }
        //printer.print((0,h+1), &self.buffer);
        //printer.print((0,h+2), "abcdefghijklmnopqrstuvwxyz1234567890,.-+ABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#ü%&/()=;:_*");
        printer.print((0,h+1), &format!("len {}", self.buffer.len()))
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        Vec2::new(self.size.x, self.size.y+2) 
    }
}

impl View for TermView {
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

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        let mut t = self.c.lock().unwrap();
        t.required_size(size)
    }
}