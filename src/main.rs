#![allow(warnings)]
use crate::term_control::clear_screen;
use crate::term_control::raw_on;
use crate::term_control::restore_term;
use keypress::{Keys, listen_keypress};

use std::{
    io::{Write, stdout},
    process::exit,
};

mod keypress;
mod term_control;

fn main() {
    clear_screen();
    raw_on();
    loop {
        let ch: u8 = listen_keypress(false);
        if ch == Keys::Q_LOWER {
            break;
        } else if ch == Keys::CTRL_C {
            restore_term(); // restoring terminal before we leave
            exit(130); // 130 exit code tells the terminal that the process was interupted with Ctrl-C
        } else if ch == Keys::ESC {
            _ = listen_keypress(false);
            let ch = listen_keypress(false);
            if ch == Keys::A_KEY {
                print!("\x1b[A");
            } else if ch == Keys::B_KEY {
                print!("\x1b[B");
            } else if ch == Keys::C_KEY {
                print!("\x1b[C");
            } else if ch == Keys::D_KEY {
                print!("\x1b[D");
            }
            stdout().flush().expect("unable to flush stdout");
        }
    }
    restore_term();
}
