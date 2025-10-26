# Chapter 03 "Keypress"

Now that are done with the setup. Let's recap what we learnt:

- Turning Raw Mode on and off
- Clearing Terminal
- Capturing the character typed by User

In this chapter we will learn how to process each keypress. Here is the overview of the topics covered in this chapter:

1. Cursor Movement
2. Special Key handling 
3. Key listening

## Cursor Movement
This is simple we want the cursor to move where we tell it to by giving it x and y coordinates ([Cartesian Plane](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://en.wikipedia.org/wiki/Cartesian_coordinate_system&ved=2ahUKEwjS45rawLyQAxUXSfEDHZ5rGsYQFnoECEMQAQ&usg=AOvVaw1PZYiU9lw_8g_5dbB9ADv5)) Or By simply pressing Arrow keys but you see Arrow keys don't actually send one single signal like if we press 'q' it sends 113 which we can easily put in an if statement like:

```rs
let char = get_char();
if char == 113{
    do_stuff();
}
```

Lets take Up Arrow as an example it sends three signals which are `27`, `91` and Then `65` these are all [ascii codes](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://en.wikipedia.org/wiki/ASCII&ved=2ahUKEwjOooTIw7yQAxVTQaQEHZA3GZUQFnoECB0QAQ&usg=AOvVaw2HzUa6hy2uH7luG7ejFlib) as characters they look like (27 is non printable as char so I wrote it here as NP meaning Non Printable) `NP[A`. So firstly we have to listen for the NP character like so:

```rs
fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false); // I renamed get_char to listen_stdin for clarity
        if (ch == 113) {
            break;
        }

        if (ch == 27) {}
    }
    restore_term();
}
```
Now we will listen for two more characters `[` and `A` so:
```rs
fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false);
        if (ch == 113) {
            break;
        }

        if (ch == 27) {
            _ = listen_stdin(false); // We have no use to the [ so we will just waste it.
            let ch = listen_stdin(false);
            if (ch == 65) {}
        }
    }
    restore_term();
}
```
Now finally we will add the functionality to this. We want to make the cursor go up a line when we press A. We will use this escape for `\x1b[A` This is pretty simple to understand `\x1b` means that an escape sequence is about be given `[A` is the actually functionality we want which is just that we are literally telling the terminal that Up Arrow aka 91, 65 aka [A has been pressed.
```rs
fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false);
        if (ch == 113) {
            break;
        }

        if (ch == 27) {
            _ = listen_stdin(false);
            let ch = listen_stdin(false);
            if (ch == 65) {
                print!("\x1b[A");
                stdout().flush().expect("unable to flush stdout");
            }
        }
    }
    restore_term();
}
```
A quick note on what is `stdout().flush` normally the termnial output meaning stdout in this case only prints things when it encounters a new line but in this case we don't want to print a newline so we will manully tell the stdout to flush itself.

Now we will just repeat this process for the other Keys which are `NP[B` for Down Key, `NP[C` for Right Key and `NP[D` for Left Key. 

```rs
fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false);
        if (ch == 113) {
            break;
        }

        if (ch == 27) {
            _ = listen_stdin(false);
            let ch = listen_stdin(false);
            if (ch == 65) {
                print!("\x1b[A");
                
            } else if (ch == 66) {
                print!("\x1b[B");
                
            } else if (ch == 67) {
                print!("\x1b[C");
                
            } else if (ch == 68) {
                print!("\x1b[D");
            }
            
            stdout().flush().expect("unable to flush stdout");
        }
    }
    restore_term();
}
```

we will make a function to go to a specific coordinate later on.

## Special Key handling
While working in a terminal there are some special keys or commands that we give like `Ctrl-C` to kill or interupt a process and abort the current task and regain user control. This is the first thing we want to handle

```rs
fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false);
        if (ch == 113) {
            break;
        } else if (ch == 3) {
            restore_term(); // restoring terminal before we leave
            exit(130); // 130 exit code tells the terminal that the process was interupted with Ctrl-C
        } else if (ch == 27) {
            _ = listen_stdin(false);
            let ch = listen_stdin(false);
            if (ch == 65) {
                print!("\x1b[A");
            } else if (ch == 66) {
                print!("\x1b[B");
            } else if (ch == 67) {
                print!("\x1b[C");
            } else if (ch == 68) {
                print!("\x1b[D");
            }
            stdout().flush().expect("unable to flush stdout");
        }
    }
    restore_term();
}
```
## Cleanup
Now we have completed most of our objectives but now lets clean this up rather than manually typing out the ascii code for each key I would rather make some enums for this like so:

```rs
use std::{
    fs::File,
    io::{BufReader, Read, Write, stdin, stdout},
    process::{Command, Output, Stdio, exit},
};

#[repr(u8)]
enum Keys {
    // --- Special Control Keys ---
    CTRL_C = 3,
    ESC = 27,

    // --- Printable ASCII Characters (32–126) ---
    SPACE = 32,
    EXCLAMATION = 33,
    DOUBLE_QUOTE = 34,
    HASH = 35,
    DOLLAR = 36,
    PERCENT = 37,
    AMPERSAND = 38,
    SINGLE_QUOTE = 39,
    LEFT_PAREN = 40,
    RIGHT_PAREN = 41,
    ASTERISK = 42,
    PLUS = 43,
    COMMA = 44,
    MINUS = 45,
    PERIOD = 46,
    SLASH = 47,

    ZERO = 48,
    ONE = 49,
    TWO = 50,
    THREE = 51,
    FOUR = 52,
    FIVE = 53,
    SIX = 54,
    SEVEN = 55,
    EIGHT = 56,
    NINE = 57,

    COLON = 58,
    SEMICOLON = 59,
    LESS_THAN = 60,
    EQUAL = 61,
    GREATER_THAN = 62,
    QUESTION = 63,
    AT = 64,

    A_KEY = 65,
    B_KEY = 66,
    C_KEY = 67,
    D_KEY = 68,
    E_KEY = 69,
    F_KEY = 70,
    G_KEY = 71,
    H_KEY = 72,
    I_KEY = 73,
    J_KEY = 74,
    K_KEY = 75,
    L_KEY = 76,
    M_KEY = 77,
    N_KEY = 78,
    O_KEY = 79,
    P_KEY = 80,
    Q_KEY = 81,
    R_KEY = 82,
    S_KEY = 83,
    T_KEY = 84,
    U_KEY = 85,
    V_KEY = 86,
    W_KEY = 87,
    X_KEY = 88,
    Y_KEY = 89,
    Z_KEY = 90,

    LEFT_BRACKET = 91,
    BACKSLASH = 92,
    RIGHT_BRACKET = 93,
    CARET = 94,
    UNDERSCORE = 95,
    GRAVE = 96,

    A_LOWER = 97,
    B_LOWER = 98,
    C_LOWER = 99,
    D_LOWER = 100,
    E_LOWER = 101,
    F_LOWER = 102,
    G_LOWER = 103,
    H_LOWER = 104,
    I_LOWER = 105,
    J_LOWER = 106,
    K_LOWER = 107,
    L_LOWER = 108,
    M_LOWER = 109,
    N_LOWER = 110,
    O_LOWER = 111,
    P_LOWER = 112,
    Q_LOWER = 113,
    R_LOWER = 114,
    S_LOWER = 115,
    T_LOWER = 116,
    U_LOWER = 117,
    V_LOWER = 118,
    W_LOWER = 119,
    X_LOWER = 120,
    Y_LOWER = 121,
    Z_LOWER = 122,

    LEFT_BRACE = 123,
    PIPE = 124,
    RIGHT_BRACE = 125,
    TILDE = 126,

    // --- Special Keys (non-ASCII but common terminal input) ---
    ARROW_UP = 128,
    ARROW_DOWN = 129,
    ARROW_RIGHT = 130,
    ARROW_LEFT = 131,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().expect("Could Not flush stdout");
}

fn restore_term() {
    let mut child = Command::new("stty")
        .arg("sane")
        .spawn()
        .expect("Unable to execute stty sane Command");
    child.wait();
}

fn raw_on() {
    let mut child = Command::new("stty")
        .arg("raw")
        .arg("-echo")
        .spawn()
        .expect("Unable to execute stty Command");
    child.wait();
}

fn listen_stdin(tell: bool) -> u8 {
    let mut buf = [0; 1];
    stdin().read(&mut buf).unwrap();
    if tell {
        if buf[0] != 27 {
            print!("character = {}\n\rcode = {}\n\r", buf[0] as char, buf[0]);
        } else {
            print!("character = (non printable)\n\rcode = {}\n\r", buf[0]);
        }
    }
    return buf[0];
}

fn main() {
    clear_screen();
    raw_on();
    while (true) {
        let ch: u8 = listen_stdin(false);
        if (ch == 113) {
            break;
        } else if (ch == 3) {
            restore_term(); // restoring terminal before we leave
            exit(130); // 130 exit code tells the terminal that the process was interupted with Ctrl-C
        } else if (ch == 27) {
            _ = listen_stdin(false);
            let ch = listen_stdin(false);
            if (ch == 65) {
                print!("\x1b[A");
            } else if (ch == 66) {
                print!("\x1b[B");
            } else if (ch == 67) {
                print!("\x1b[C");
            } else if (ch == 68) {
                print!("\x1b[D");
            }
            stdout().flush().expect("unable to flush stdout");
        }
    }
    restore_term();
}
```
Full disclosure I generated the enum table with chatgpt because it should be used to do the stupid repetitive shit.
