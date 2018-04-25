#![allow(dead_code)]

use arch::device::port;
use spin::Mutex;

lazy_static! {
    static ref KEYBOARD_STATE: Mutex<Keyboard> = Mutex::new(Keyboard {
        left_shift: false,
        right_shift: false
    });
}

pub fn key_handler() {
    let data = port::inb(0x60);
    let (scancode, pressed) = if data >= 0x80 {
        (data - 0x80, false)
    } else {
        (data, true)
    };

    let mut keyboard = KEYBOARD_STATE.lock();
    if scancode == 0x2A {
        keyboard.left_shift = pressed;
    } else if scancode == 0x36 {
        keyboard.right_shift = pressed;
    }

    let character = get_char(scancode, keyboard.left_shift | keyboard.right_shift);

    if pressed == true && character != '\0' {
        print!("{}", character);
    }

    // Send EOI
    ::arch::device::pic::eoi(33);
}

pub struct Keyboard {
    left_shift: bool,
    right_shift: bool,
}

static US: [[char; 2]; 58] = [
    ['\0', '\0'],
    ['\x1B', '\x1B'],
    ['1', '!'],
    ['2', '@'],
    ['3', '#'],
    ['4', '$'],
    ['5', '%'],
    ['6', '^'],
    ['7', '&'],
    ['8', '*'],
    ['9', '('],
    ['0', ')'],
    ['-', '_'],
    ['=', '+'],
    ['\x7F', '\x7F'],
    ['\t', '\t'],
    ['q', 'Q'],
    ['w', 'W'],
    ['e', 'E'],
    ['r', 'R'],
    ['t', 'T'],
    ['y', 'Y'],
    ['u', 'U'],
    ['i', 'I'],
    ['o', 'O'],
    ['p', 'P'],
    ['[', '{'],
    [']', '}'],
    ['\n', '\n'],
    ['\0', '\0'],
    ['a', 'A'],
    ['s', 'S'],
    ['d', 'D'],
    ['f', 'F'],
    ['g', 'G'],
    ['h', 'H'],
    ['j', 'J'],
    ['k', 'K'],
    ['l', 'L'],
    [';', ':'],
    ['\'', '"'],
    ['`', '~'],
    ['\0', '\0'],
    ['\\', '|'],
    ['z', 'Z'],
    ['x', 'X'],
    ['c', 'C'],
    ['v', 'V'],
    ['b', 'B'],
    ['n', 'N'],
    ['m', 'M'],
    [',', '<'],
    ['.', '>'],
    ['/', '?'],
    ['\0', '\0'],
    ['\0', '\0'],
    ['\0', '\0'],
    [' ', ' '],
];

fn get_char(scancode: u8, shift: bool) -> char {
    if let Some(c) = US.get(scancode as usize) {
        if shift {
            c[1]
        } else {
            c[0]
        }
    } else {
        '\0'
    }
}
