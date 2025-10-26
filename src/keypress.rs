#![allow(warnings)]
use std::io::{Read, stdin};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Keys {
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

impl PartialEq<Keys> for u8 {
    fn eq(&self, other: &Keys) -> bool {
        *self == *other as u8
    }
}

pub fn listen_keypress(tell: bool) -> u8 {
    let mut buf = [0; 1];
    stdin().read(&mut buf).unwrap();
    if tell {
        if buf[0] != Keys::ESC {
            print!("character = {}\n\rcode = {}\n\r", buf[0] as char, buf[0]);
        } else {
            print!("character = (non printable)\n\rcode = {}\n\r", buf[0]);
        }
    }
    return buf[0];
}
