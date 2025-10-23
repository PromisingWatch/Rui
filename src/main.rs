use std::{
    io::{Read, Write, stdin, stdout},
    process::{Command, Output, exit},
};

fn get_width() -> usize {
    let w: usize;
    if let Some((w, _)) = term_size::dimensions() {
        return w;
    } else {
        println!("Unable to get term size :(");
        exit(-1);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_termatrr() -> String {
    let termattr = Command::new("stty")
        .arg("-g")
        .output()
        .expect("Unable to get terminal attributes using 'stty' command");
    let termattr = String::from_utf8_lossy(&termattr.stdout);
    let termattr = termattr.trim();
    return String::from(termattr);
}

fn restore_term(org_termattr: Output) {}

fn raw_on() {}

fn main() {
    let org_termattr: String = get_termatrr();
    // loop {
    //     // clear_screen();
    //     raw_on();
    // }
}
