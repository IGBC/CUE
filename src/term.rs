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

pub struct TermView {
    buffer: Box<[char]>, // Screen Buffer sized width * height
    size: XY<usize>, // Size of window
    cursor: XY<usize>, // Current postion of the input cursor
    fh: File,
}

impl TermView {
    /// Creates a new TermView with the given content.
    pub fn new(w: usize, h: usize, f: File) -> Self {
        let mut v = TermView {
            size: XY::new(w,h),
            buffer: vec![' '; w*h].into_boxed_slice(),
            cursor: XY::new(0,0),
            fh: f,
        };
        // this is never gonna work need to research threading
        //thread::spawn(|| { v.read_loop() });
        v
    }

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

    fn read_loop(&mut self) {
        let f = self.fh.try_clone().expect("file didn't clone");
        let mut f = f.bytes();
        loop {
            let c = f.next();
            let c = match c {
                Some(r) => r.expect("IO::ERROR"),
                None => break,
            };
            self.put_char(c as char);
        }
    }
}


impl View for TermView {
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

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            _ => return EventResult::Ignored,
        }
        EventResult::Consumed(None)
    }

    fn required_size(&mut self, size: Vec2) -> Vec2 {
        Vec2::new(self.size.x, self.size.y+2) 
    }
}