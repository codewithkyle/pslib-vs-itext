use std::time::Instant;
use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter};
use std::path::Path;

use pslib::{Document, Line, Page, Rect};

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug)]
struct Lin {
    x: f32,
    y: f32,
    l: f32,
}

// Enum to store either a Rectangle or Line
#[derive(Debug)]
enum Shape {
    Rect(Rectangle),
    Line(Lin),
}

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_text_file>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
    }

    // Open the file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    // Vector to store shapes in order
    let mut shapes = Vec::new();

    // Parse each line
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse based on the last part of each line ("rect" or "line")
        match parts.as_slice() {
            [x, y, w, h, "rect"] => {
                let rect = Rectangle {
                    x: x.parse().unwrap_or(0.0),
                    y: y.parse().unwrap_or(0.0),
                    w: w.parse().unwrap_or(0.0),
                    h: h.parse().unwrap_or(0.0),
                };
                shapes.push(Shape::Rect(rect));
            }
            [x, y, l, "line"] => {
                let line = Lin {
                    x: x.parse().unwrap_or(0.0),
                    y: y.parse().unwrap_or(0.0),
                    l: l.parse().unwrap_or(0.0),
                };
                shapes.push(Shape::Line(line));
            }
            _ => {
                eprintln!("Skipping malformed line: {}", line);
            }
        }
    }

    let path = Path::new("output.ps");
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    let file = OpenOptions::new().write(true).create(true).open(path)?;
    let writer = BufWriter::new(&file);

    let start = Instant::now();
    // Begin
    let mut doc = Document::new(writer);
    let mut page = Page::new(200, 200);
    for shape in &shapes {
        match shape {
            Shape::Rect(rect) => {
                let _ = page.add(&Rect::new(rect.x, rect.y, rect.w, rect.h).stroke_rgb(1.0, 0.0,0.0,0.0));
            }
            Shape::Line(line) => {
                let _ = page.add(&Line::new(line.x, line.y, line.l).stroke_rgb(1.0, 0.0,0.0,0.0));
            }
        }
    }
    let _ = doc.add(&page);
    let _ = doc.close();
    // Stop
    let duration = start.elapsed();
    print!("Time elapsed: {:?}", duration);

    Ok(())
}

