mod services;

use std::{ascii, fs};
use std::process::exit;
use clap::Parser;
use image::{open, DynamicImage, ImageError};
use crate::services::img_to_ascii;
use owo_colors::{OwoColorize, Rgb};
use crate::services::img_to_ascii::{img_to_colored_ascii, Color};

#[derive(Parser)]
#[command(name = "img-ascii")]
#[command(about = "Convert images to ASCII art")]
#[derive(Debug)]
struct Cli {
    input: String,

    #[arg(long, default_value="console")]
    output: String,

    #[arg(long, default_value="name")]
    name: String,

    #[arg(long)]
    file: Option<String>,

    #[arg(long, default_value="style")]
    style: String,

    #[arg(long, default_value = "80")]
    width: u32,

    #[arg(long, default_value = "false")]
    color: String
}

fn main() {
    let args = Cli::parse();

    let img = match load_image(&args.input) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Erreur lors du chargement de l'image: {}", e);
            return
        }
    };



    match args.output.as_str() {
        "console" => {
            match args.color.as_str() {
                "true" => print_colored_ascii_lines(&img_to_colored_ascii(&img, args.width, &args.style)),
                _ => print_ascii_lines(&img_to_ascii(&img, args.width, &args.style)),
            }

        },
        "file" => {
            let ascii_lines = img_to_ascii(&img, args.width, &args.style);

            fs::write(args.name, ascii_lines.join("\n"))
                .unwrap_or_else(|e| eprintln!("Erreur: {}", e))
        },
        _ => {
            let ascii_lines = img_to_ascii(&img, args.width, &args.style, );

            for ascii_line in ascii_lines {
                println!("{}",ascii_line);
            }
        }
    }
}

fn print_colored_ascii_lines(colored_lines: &Vec<Vec<(Color, char)>>) {
    for line in colored_lines {
        for (color, ch) in line {
            let rgb = Rgb(color.r, color.g, color.b);
            print!("{}", ch.color(rgb));
        }
        println!();
    }
}
fn print_ascii_lines(ascii_lines: &Vec<String>) {
    for line in ascii_lines {
        println!("{}", line);
    }
}

fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    open(path)
}
