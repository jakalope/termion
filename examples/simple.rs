extern crate libterm;

use libterm::{TermControl, raw_mode, Color};
use std::io::{Read, Write, stdout, stdin};

fn main() {
    let _raw = raw_mode();
    let mut stdout = stdout();
    let stdin = stdin();

    stdout.goto(5, 5).unwrap();
    stdout.clear().unwrap();
    stdout.write(b"yo, 'q' will exit.").unwrap();
    stdout.flush().unwrap();
    stdout.goto(20, 10).unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            b'q' => return,
            b'c' => stdout.clear(),
            b'r' => stdout.color(Color::Rgb(5, 0, 0)),
            a => stdout.write(&[a]),
        }.unwrap();

        stdout.flush().unwrap();
    }
}
