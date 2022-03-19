#![feature(let_else)]
#![feature(let_chains)]

use owo_colors::{OwoColorize, Style, colors::css::*};
use std::{fs::read, vec::IntoIter, io::{stdout, Write}};

fn main() {
    let Some(filename) = std::env::args().nth(1) else {
        println!("Expected file name argument.");
        std::process::exit(1);
    };

    let bytecode = read(&filename).unwrap();

    let mut file = bytecode.clone().into_iter();

    // Declaration of styles.
    // These can be ignored.
    let noop = Style::new()
        .white()
        .on_black();

    let pix = Style::new()
        .white()
        .on_green();

    let cpix = Style::new()
        .black()
        .on_bright_green();
    
    let spr = Style::new()
        .black()
        .on_bright_cyan();
    
    let flsh = Style::new()
        .black()
        .on_white();
    
    let math = Style::new()
        .black()
        .on_red();
    
    let var = Style::new()
        .bg::<Pink>()
        .fg::<Black>();
    
    let jmp = Style::new()
        .black()
        .on_bright_purple();
    
    let inp = Style::new()
        .black()
        .on_bright_yellow();
    

    // Prints out header data, as it is separate
    // from everything else
    println!("[{}]", "Header data".fg::<Gray>().on_white());
    while let Some(val) = file.next() && val != 0x00 {
        match val {
            // Render game title
            0x01 => {
                let mut title = String::new();

                while let Some(val) = file.next() && val != 0x01 {
                    if val == 0x00 {
                        title.push(file.next().unwrap() as char)
                    }
                    title.push(val as char);
                }

                println!("{} {}", "Game title:".white().on_purple(), title)
            },
            0x02 => {
                println!("{}", "Keep Looped".white().on_green());
            }
            0x03 => {
                println!("{}", "Alt pallette".black().on_bright_cyan())
            }
            0x04 => {
                println!("{}", "Keep Open".white().on_cyan());
            }
            // inp styling is black on yellow, which is eye-catching.
            // not necessarily pertaining to input.
            any => println!("Unknown header: {}", any.style(inp))
        }
    }
    stdout().flush().unwrap();
    println!("[{}{}]", "END OF ".style(inp), "Header data".fg::<Gray>().on_white());

    while let Some(val) = file.next() {
        macro_rules! chunk {
            ($style: expr, $name: expr, $size: expr) => {
                chunk($style, val, $name, $size, &mut file, &bytecode)
            }
        }

        match val {
            0x00 => chunk!(noop, "noop", 0),
            0x01 => chunk!(pix, "pix", 3),
            0x02 => chunk!(cpix, "cpix", 3),
            0x03 => chunk!(spr, "spr", 10),
            0xfc => chunk!(spr, "cls", 1),
            0xfb => chunk!(flsh, "flsh", 0),
            0xf0 => chunk!(math, "fdiv", 3),
            0xf1 => chunk!(math, "fsub", 3),
            0xf2 => chunk!(math, "fadd", 3),
            0xf3 => chunk!(math, "fmul", 3),
            0xf4 => chunk!(math, "div", 3),
            0xf5 => chunk!(math, "sub", 3),
            0xf6 => chunk!(math, "add", 3),
            0xf7 => chunk!(math, "mul", 3),
            0xb0 => chunk!(math, "not", 2),
            0xb1 => chunk!(math, "gt", 3),
            0xb2 => chunk!(math, "lt", 3),
            0xa1 => chunk!(var, "var", 10),
            0xa2 => chunk!(var, "let", 10),
            0xa3 => chunk!(var, "arrw", 3),
            0xe1 => chunk!(jmp, "tjmp", 9),
            0xe2 => chunk!(jmp, "fjmp", 9),
            0xe3 => chunk!(jmp, "jmp", 8),
            0xd0 => chunk!(inp, "key", 2),
            a => chunk!(inp, "!! UNKNOWN INSTRUCTION !!", 0)
        }
    }
}

pub fn chunk(style: Style, inst: u8, name: &str, size: usize, iter: &mut IntoIter<u8>, src: &[u8]) {
    let mut out: Vec<u8> = vec![];

    for _ in 0..size {
        let Some(val) = iter.next() else {
            println!("Unexpected EOF (incomplete instruction {inst:0>2x}!)");
            std::process::exit(1);
        };

        out.push(val);
    }

    println!("({name:>5}) {:0>2.15x} :: {:0>2x?} {}", inst.style(style), out.style(style), if let 0xe1 | 0xe2 | 0xe3 = inst {

        let bytes = u64::from_le_bytes([
            out[0],
            out[1],
            out[2],
            out[3],
            out[4],
            out[5],
            out[6],
            out[7]
        ]);

        format!("@ {bytes}")
    } else {
        format!("")
    });
}