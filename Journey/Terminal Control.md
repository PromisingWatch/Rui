## Terminal Control 
First things first I want to know how to do the following:

- Clear Terminal Screen
- Enter **Raw** Mode 
- Move Cursor
- Get Terminal Width and Height

### Clearing the Terminal Screen
#### Attempt 1 "Clear"
When I think of clearing the screen I think of the `clear` command in the shell (fish in my case btw). So I hope that by researching how the clear command is implemented I can learn how the terminal is cleared. Okay so after some digging I was going down a hole maybe I shouldn't because a much simpler alternative exists. 
#### Attempt 2 "Escape Codes"
I can't believe I forgot about this. 

>Escape Sequences are basically special characters you give to your terminal on how to behave depending on what type of escape sequence you give. 

They start with a '\' character and then according to the functionality you want you would give the second character. You might have run into this when trying to print newlines in `C` here is an example to illustrate the point I am making:
```c
#include <stdio.h>
void main(void){
    printf("Hello World\n");
}
```
The `\n` is an escape sequence which tells the terminal to move the cursor to the next line (Small Note on `\n`'s that will be more important later on. In most modern systems the `\n` is the same as `\n\r`. Just make a mental note of this). 
They work in basically all languages I know of (which to be fair are like only `C`, `C++` and `Rust`). So to clear the screen we would do the following:
```rs
use std::{
    io::stdout, // Used for controlling the standard Output
};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main(){
        clear_screen();
}
```
Okay so most of this is pretty self-explanitory. We used the `\x1B[2J` and `\x1B[1;1H` escape sequences to clear the screen and move cursor to the top left side of the terminal. Here is they mean:

| Value | Meaning |
| ---   |  ---    |
| \x1B  | This is a Hexadecimal Value             |
| [2J   | Clear the Screen                        |
| [1;1H | Move Cursor to Top left of the terminal |

#### Entering Raw Mode 
What is Raw Mode and What Cooked Mode? Well before explaining that we have to talk about **stdout**. 
##### Stdout 
You probably have heard this phrase of "Writing to standard output" but have you ever thought of what it means? for that we will have to know what stdout even is. (In Unix Systems not sure about windows) standard output is a [stream](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://en.wikipedia.org/wiki/Stream_(computing)&ved=2ahUKEwi50deT_beQAxXkUaQEHcgDAcAQFnoECBgQAQ&usg=AOvVaw18aqkP6D01VQdDjdovCxPz) which is the default for output of programs. Unless it is purposefully redirected it usually just inherits from the parent program which is usually a terminal. 
##### Cooked Mode 
If you have ever worked in a terminal before (I am assuming you have if you haven't what are you doing here?) then you know that when you write a command it does not get executed until you press Enter aka "Carrage Return". This is called being in "Cooked Mode".
##### Raw Mode 
Raw mode is the opposite of Cooked Mode. In this mode ther terminal immediately gives the input from user to the program handeling the input at that time so in out case that would be our rust program. Also it does not echo characters.
##### Implementation 
**Attempt 01 "stdlib"**: \
Now finally we can write our code. If the rust stdlib allowed us to to do it that is. yep. fellas the rust stdlib does not give any APIs to manage the terminal on a lower level. So if I had a sane mind and was more willing to submit defeat I would hang up this project and just use `crossterm` or `libc` if I was feeling rather nostalgic BUT! I am a masochist. \
**Attempt 02 "Shell Commands"**: \
So the rust language itself does not provide us with a way to interact with the attributes of the terminal. So we'll just use shell commands to do the same thing. You see rust gives use a way to send commands to the OS by using the [process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) which is technically a part of the stdlib. We use process::Command in the following way:
```rust
use std::process::Command;

    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process")
```

We will first save the terminal's normal attributes('default settings') and then enable the raw mode functionality and after we are done we will then restore the terminal's normal state which is cooked mode. \
We would save the terminal settings in the following way:
```rs
fn save_termattrs() -> std::io::Result<()> {
    let file = File::create("temp.rui").expect("Unable to Open file");
    let mut child = Command::new("stty")
        .arg("-g")
        .stdout(Stdio::from(file))
        .spawn()
        .expect("Unable to execute stty Command");
    child.wait();
    Ok(())
}

fn main(){
    save_termattrs();
}
```
In this `save_termattrs()` function we firstly create a file called `temp.rui` this is the file in which we will store our terminal attributes. After that we Create a new process and call the `stty` command with `-g` flag which will output the terminal attributes in a stty readable format. something like this: `4100:5:f00bf:8a3b:3:1c:7f:15:4:0:1:0:11:13:1a:0:12:f:17:16:0:0:0:0:0:0:0:0:0:0:0:0:0:0:0:0` we will also give the stdout for this process to the file instead of the normal stdout(terminal screen). Since this process is async we will have to wait for the child to finish the execution of stty command.  \
Now we will start to enable raw mode finally. Lets make another function here:
```rs
fn save_termattrs() -> std::io::Result<()> {
    let file = File::create("temp.rui").expect("Unable to Open file");
    let mut child = Command::new("stty")
        .arg("-g")
        .stdout(Stdio::from(file))
        .spawn()
        .expect("Unable to execute stty Command");
    child.wait();
    Ok(())
}

fn raw_on() {}

fn main() {
    save_termattrs();
}
```
This function will take no argument and just turn on raw mode. firstly we will create another process.
```rs
fn raw_on(){
    let mut child = Command::new("stty")
        .arg("-echo") // Turn off echoing
        .arg("raw")   // Turn on raw mode 
        .spawn()
        .expect("Unable to execute stty Command");
    child.wait();
}
```
and thats it! we made the terminal go into raw mode you will see if you try this on your machine that the characters you type will not show up on the screen but rest assured that they are still being written. try to type `exit` after running this you will see that the program you were running will close (mostly likely the shell of your choice).  

> My dear fish users I myself use it but turns out it completely ignores what stty says to do as soon as the process ends so just use bash for this one please ^-^.

> After digging on some examples on how stty is used I found out that we don't need to save the terminal settings as stty has an argument which we can give it called sane. we just call stty sane and everything goes back to the way it was.

here is our new code:
```rs
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

fn main() {
    raw_on();
}
Now to test it out. Lets make a buffer with one element space and read from the `stdin`:
```rs
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

fn main() {
    raw_on();

    let mut buf = [0; 1];
    loop {
        stdin().read(&mut buf).unwrap();
        print!("character = {}\n\rcode = {}\n\r", buf[0] as char, buf[0]);
        if (buf[0] == 113) { // quit out of loop is the input char is 'q'
            break;
        }
    }
    restore_term();
}
```
This program when you run it will give you the character you entered and the ascii code for it as well. This section is done. Now let me just put this into a nice `get_char()` function.
```rs
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

fn get_char(tell: bool) -> u8 {
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
        let ch: u8 = get_char(true);
        if (ch == 113) {
            break;
        }
    }
    restore_term();
}
```
