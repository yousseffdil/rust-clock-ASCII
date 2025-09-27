use std::thread;
use std::time::Duration;
use std::env;
use chrono::{Local, Timelike};

fn main() {
    let args: Vec<String> = env::args().collect();
    let show_seconds = args.contains(&"-s".to_string());
    
    loop {
        print!("\x1B[2J\x1B[1;1H");
        
        let now = Local::now();
        let hours = now.hour();
        let minutes = now.minute();
        let seconds = now.second();
        
        let time_str = if show_seconds {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", hours, minutes)
        };
        
        display_time_with_blocks(&time_str);
        
        thread::sleep(Duration::from_secs(1));
    }
}

fn display_time_with_blocks(time_str: &str) {
    let patterns = [
        ["███", "█ █", "█ █", "█ █", "███"], // 0
        [" █ ", " █ ", " █ ", " █ ", " █ "], // 1
        ["███", "  █", "███", "█  ", "███"], // 2
        ["███", "  █", "███", "  █", "███"], // 3
        ["█ █", "█ █", "███", "  █", "  █"], // 4
        ["███", "█  ", "███", "  █", "███"], // 5
        ["███", "█  ", "███", "█ █", "███"], // 6
        ["███", "  █", "  █", "  █", "  █"], // 7
        ["███", "█ █", "███", "█ █", "███"], // 8
        ["███", "█ █", "███", "  █", "███"], // 9
    ];
    
    let colon = ["   ", " █ ", "   ", " █ ", "   "];
    
    for line in 0..5 {
        for ch in time_str.chars() {
            if ch == ':' {
                print!("{} ", colon[line]);
            } else if let Some(digit) = ch.to_digit(10) {
                print!("{} ", patterns[digit as usize][line]);
            }
        }
        println!();
    }
}
