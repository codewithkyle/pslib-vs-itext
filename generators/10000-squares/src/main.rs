use std::env;
use std::fs::File;
use std::io::{self, Write};
use rand::Rng;
use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <number_of_rectangles> <number_of_lines>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid arguments"));
    }

    // Parse the inputs for rectangles and lines
    let num_rectangles: usize = args[1].parse().expect("Invalid number for rectangles");
    let num_lines: usize = args[2].parse().expect("Invalid number for lines");

    // Create a vector to hold all shapes
    let mut shapes = Vec::new();
    let mut rng = rand::thread_rng();

    // Generate rectangles
    for _ in 0..num_rectangles {
        let x: f64 = rng.gen_range(0.0..100.0);
        let y: f64 = rng.gen_range(0.0..100.0);
        let w: f64 = rng.gen_range(1.0..50.0);
        let h: f64 = rng.gen_range(1.0..50.0);
        shapes.push(format!("{:.2} {:.2} {:.2} {:.2} rect", x, y, w, h));
    }

    // Generate lines
    for _ in 0..num_lines {
        let x: f64 = rng.gen_range(0.0..100.0);
        let y: f64 = rng.gen_range(0.0..100.0);
        let l: f64 = rng.gen_range(1.0..100.0);
        shapes.push(format!("{:.2} {:.2} {:.2} line", x, y, l));
    }

    // Shuffle shapes vector
    shapes.shuffle(&mut rng);

    // Create output file and write shuffled shapes
    let mut file = File::create("output.txt")?;
    for shape in shapes {
        writeln!(file, "{}", shape)?;
    }

    println!("File 'output.txt' generated successfully with mixed shapes.");
    Ok(())
}

