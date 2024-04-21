use image::{Rgb, RgbImage};
use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("Digite o nome do arquivo do wallpaper:");
    let mut filename = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filename).expect("Failed to read filename");
    let filename = filename.trim(); 

    let width = 1920;
    let height = 1080;

    let mut img = RgbImage::new(width, height);

    let mut rng = rand::thread_rng();

    let frequency_x: f64 = rng.gen_range(0.05..0.15);
    let frequency_y: f64 = rng.gen_range(0.05..0.15);

    let phase_shift_x: f64 = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    let phase_shift_y: f64 = rng.gen_range(0.0..std::f64::consts::PI * 2.0);

    let amplitude: f64 = rng.gen_range(50.0..150.0);

    for y in 0..height {
        for x in 0..width {
            let wave_color = generate_wave_color(x, y, width, height, frequency_x, frequency_y, phase_shift_x, phase_shift_y, amplitude, &mut rng);
            img.put_pixel(x, y, Rgb(wave_color));
        }
    }

    let full_filename = format!("{}.png", filename);
    img.save(&full_filename).expect("Failed to save image");

    println!("Wallpaper gerado com sucesso como '{}'", full_filename);
}

fn generate_wave_color(x: u32, y: u32, width: u32, height: u32, frequency_x: f64, frequency_y: f64, phase_shift_x: f64, phase_shift_y: f64, amplitude: f64, rng: &mut ThreadRng) -> [u8; 3] {
    let x_wave = (x as f64 * frequency_x * std::f64::consts::PI * 2.0 / width as f64 + phase_shift_x).sin();
    let y_wave = (y as f64 * frequency_y * std::f64::consts::PI * 2.0 / height as f64 + phase_shift_y).sin();

    let r: u8 = rng.gen_range(0..=255);
    let g: u8 = rng.gen_range(0..=255);
    let b: u8 = rng.gen_range(0..=255);

    let r = (r as f64 + amplitude * x_wave) as u8;
    let g = (g as f64 + amplitude * y_wave) as u8;
    let b = (b as f64 + amplitude * (x_wave * y_wave)) as u8;

    [r, g, b]
}
