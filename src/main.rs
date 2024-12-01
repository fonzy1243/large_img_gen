use rand::Rng;
use std::fs::File;
use std::io::{Result, Write};

fn gen_img(width: usize, height: usize, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    let mut rng = rand::thread_rng();

    writeln!(file, "{} {}", width, height)?;

    for _ in 0..height {
        let row: Vec<String> = (0..width)
            .map(|_| format!("{:.4}", rng.gen::<f32>()))
            .collect();

        writeln!(file, "{}", row.join(" "))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    gen_img(2560, 1440, "float_img.txt")
}
