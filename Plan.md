# What is This?
This is a minimalistic Tui toolkit. It is written completely in Rust. It has ZERO external dependencies {except for the stdlib :)}. This is a newbie made project so it might have some shitty code and rookie mistakes and I would love any feedback you have that isn't 'Get Gud' . I am documenting this as I go along so that I can record where I mess up. This is off the cuff I am not going to make a proper script for this so don't blame me if this is scuffed as hell. Also since this is an educational project so I will not use any LLM to generate code. Any code you see will be either from my dome or an online source which is not google Gemini answers.

# The Roadmap
These is my current road map:
1. Learn Terminal basics like clearing the screen.
2. Learn to capture and deal with each key press.
3. Building a minimalistic rendering engine.
4. Small UI framework on top of the renderer.
5. Making and handling Event loop and State Management.

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
Now finally we can write our code. If the rust stdlib allowed us to to do it that is. yep. fellas the rust stdlib does not give any APIs to manage the terminal on a lower level. So if I had a sane mind and was more willing to submit defeat I would hang up this project and just use `crossterm` or `libc` if I was feeling rather nostalgic BUT! I am a masochist.
**Attempt 02 "Shell Commands"**: \
So the rust language itself does not provide us with a way to interact with the attributes of the terminal. So we'll just use shell commands to do the same thing. You see rust gives use a way to send commands to the OS by using the [process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) which is technically a part of the stdlib. We use process::Command in the following way:
```rust
use std::process::Command;

let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .expect("failed to execute process")
} else {
    Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process")
};

let hello = output.stdout;
```
