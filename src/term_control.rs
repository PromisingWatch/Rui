#![allow(warnings)]
use std::{
    io::{Write, stdout},
    process::Command,
};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().expect("Could Not flush stdout");
}

pub fn restore_term() {
    let mut child = Command::new("stty")
        .arg("sane")
        .spawn()
        .expect("Unable to execute stty sane Command");
    child
        .wait()
        .expect("Unable to wait for stty sane child process");
}

pub fn raw_on() {
    let mut child = Command::new("stty")
        .arg("raw")
        .arg("-echo")
        .spawn()
        .expect("Unable to execute stty Command");
    child.wait();
}
