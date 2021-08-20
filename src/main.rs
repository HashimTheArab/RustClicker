use std::{thread, io};
use winapi::um::winuser::{MOUSEEVENTF_LEFTUP, MOUSEEVENTF_LEFTDOWN};
use winapi::um::winuser;
use std::io::Write;
use std::thread::sleep;
use std::time::{SystemTime};
use std::alloc::System;

mod hook;

/*
Sorry for the shit code this is my second time using rust :)
 */

// http://javascriptkeycode.com/
const START_KEY: u32 = 114;
const END_KEY: u32 = 114;

static mut STARTED: bool = false; // sorry for my sins

fn main() {
    print!("CPS: ");
    io::stdout().flush().expect("Error flushing stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");

    let cps: f64 = input.trim().parse().unwrap();

    thread::spawn(move || unsafe {
        let mut time: SystemTime = SystemTime::now();
        loop {
            if STARTED {
                if SystemTime::now().duration_since(time).unwrap().as_secs_f64() > 1.0 / cps {
                    winuser::mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
                    winuser::mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                    time = SystemTime::now();
                }
            }
        }
    });

    unsafe {
        hook::new(|_|{}, key_down);
    }
}

fn key_down(code: u32){
    unsafe {
        if code == START_KEY && !STARTED {
            STARTED = true;
        } else if code == END_KEY && STARTED {
            STARTED = false;
        }
    }
}