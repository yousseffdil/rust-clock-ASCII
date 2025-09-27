use std::thread;
use std::time::Duration;
use std::env;
use chrono::{Local, Timelike, Datelike};
use std::io::{self, Write};

// Colores ANSI
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";
const MAGENTA: &str = "\x1B[35m";
const CYAN: &str = "\x1B[36m";
const WHITE: &str = "\x1B[37m";
const RESET: &str = "\x1B[0m";

#[derive(Clone, Copy)]
enum ClockStyle {
    Block,
    Digital,
    Minimal,
}

struct Config {
    show_seconds: bool,
    show_date: bool,
    color: &'static str,
    style: ClockStyle,
    show_12hour: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config {
        show_seconds: args.contains(&"-s".to_string()) || args.contains(&"--seconds".to_string()),
        show_date: args.contains(&"-d".to_string()) || args.contains(&"--date".to_string()),
        color: if args.contains(&"--red".to_string()) { RED }
               else if args.contains(&"--green".to_string()) { GREEN }
               else if args.contains(&"--yellow".to_string()) { YELLOW }
               else if args.contains(&"--blue".to_string()) { BLUE }
               else if args.contains(&"--magenta".to_string()) { MAGENTA }
               else if args.contains(&"--cyan".to_string()) { CYAN }
               else { WHITE },
        style: if args.contains(&"--digital".to_string()) { ClockStyle::Digital }
               else if args.contains(&"--minimal".to_string()) { ClockStyle::Minimal }
               else { ClockStyle::Block },
        show_12hour: args.contains(&"--12h".to_string()),
    };
    
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        show_help();
        return;
    }
    
    // Ocultar cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
    
    // Restaurar cursor al salir
    ctrlc::set_handler(|| {
        print!("\x1B[?25h"); // Mostrar cursor
        io::stdout().flush().unwrap();
        std::process::exit(0);
    }).unwrap_or_else(|_| {
        // Si ctrlc no está disponible, continúa sin manejo de señales
    });
    
    loop {
        // Limpiar pantalla y posicionar cursor
        print!("\x1B[2J\x1B[1;1H");
        
        let now = Local::now();
        
        // Mostrar fecha si está habilitada
        if config.show_date {
            let date_str = format!("{}/{:02}/{:02}", 
                now.day(), now.month(), now.year());
            println!("{}{}{}\n", config.color, date_str, RESET);
        }
        
        let mut hours = now.hour();
        let minutes = now.minute();
        let seconds = now.second();
        let mut am_pm = "";
        
        // Convertir a formato 12 horas si está habilitado
        if config.show_12hour {
            am_pm = if hours >= 12 { " PM" } else { " AM" };
            if hours > 12 {
                hours -= 12;
            } else if hours == 0 {
                hours = 12;
            }
        }
        
        let time_str = if config.show_seconds {
            format!("{:02}:{:02}:{:02}{}", hours, minutes, seconds, am_pm)
        } else {
            format!("{:02}:{:02}{}", hours, minutes, am_pm)
        };
        
        match config.style {
            ClockStyle::Block => display_time_with_blocks(&time_str, config.color),
            ClockStyle::Digital => display_digital_time(&time_str, config.color),
            ClockStyle::Minimal => display_minimal_time(&time_str, config.color),
        }
        
        // Mostrar información adicional
        if config.show_date {
            let weekday = match now.weekday().number_from_monday() {
                1 => "Lunes",
                2 => "Martes", 
                3 => "Miércoles",
                4 => "Jueves",
                5 => "Viernes",
                6 => "Sábado",
                7 => "Domingo",
                _ => "Desconocido",
            };
            println!("\n{}{}{}", config.color, weekday, RESET);
        }
        
        // Mostrar zona horaria
        println!("\n{}🌍 Zona: {}{}", config.color, now.format("%Z"), RESET);
        
        thread::sleep(Duration::from_secs(1));
    }
}

fn show_help() {
    println!("Reloj Digital ASCII");
    println!("\nUso: reloj [opciones]");
    println!("\nOpciones:");
    println!("  -s, --seconds     Mostrar segundos");
    println!("  -d, --date        Mostrar fecha");
    println!("  --12h             Formato 12 horas");
    println!("  --digital         Estilo digital");
    println!("  --minimal         Estilo minimalista");
    println!("  --red             Color rojo");
    println!("  --green           Color verde");
    println!("  --yellow          Color amarillo");
    println!("  --blue            Color azul");
    println!("  --magenta         Color magenta");
    println!("  --cyan            Color cian");
    println!("  -h, --help        Mostrar esta ayuda");
    println!("\nEjemplos:");
    println!("  reloj -s -d --green    # Segundos, fecha y color verde");
    println!("  reloj --12h --blue     # Formato 12h y color azul");
    println!("  reloj --minimal --cyan # Estilo minimalista y color cian");
}

fn display_time_with_blocks(time_str: &str, color: &str) {
    let patterns = [
        ["███", "█ █", "█ █", "█ █", "███"], // 0
        [" █ ", "██ ", " █ ", " █ ", "███"], // 1
        ["███", "  █", "███", "█  ", "███"], // 2
        ["███", "  █", "███", "  █", "███"], // 3
        ["█ █", "█ █", "███", "  █", "  █"], // 4
        ["███", "█  ", "███", "  █", "███"], // 5
        ["███", "█  ", "███", "█ █", "███"], // 6
        ["███", "  █", "  █", "  █", "  █"], // 7
        ["███", "█ █", "███", "█ █", "███"], // 8
        ["███", "█ █", "███", "  █", "███"], // 9
    ];
    
    let colon = ["   ", " ● ", "   ", " ● ", "   "];
    let space = ["   ", "   ", "   ", "   ", "   "];
    
    for line in 0..5 {
        print!("{}", color);
        for ch in time_str.chars() {
            match ch {
                ':' => print!("{} ", colon[line]),
                ' ' => print!("{} ", space[line]),
                'A' | 'P' | 'M' => {
                    // Mostrar AM/PM en la línea del medio
                    if line == 2 {
                        print!("{}", ch);
                    } else {
                        print!(" ");
                    }
                },
                _ => {
                    if let Some(digit) = ch.to_digit(10) {
                        print!("{} ", patterns[digit as usize][line]);
                    }
                }
            }
        }
        println!("{}", RESET);
    }
}

fn display_digital_time(time_str: &str, color: &str) {
    println!("{}┌─────────────────────┐", color);
    println!("│     {}   │", time_str);
    println!("└───────────────────────┘{}", RESET);
}

fn display_minimal_time(time_str: &str, color: &str) {
    println!("{}⏰ {}{}", color, time_str, RESET);
}