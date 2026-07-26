use image::RgbImage;
use std::{env, error::Error, path::Path};

fn is_water(pixel: &[u8; 3]) -> bool {
    let [r, g, b] = *pixel;
    b as f32 > r as f32 * 1.12 && g as f32 > r as f32 * 1.08 && r < 105
}

fn luminance(pixel: &[u8; 3]) -> f64 {
    pixel[0] as f64 * 0.2126 + pixel[1] as f64 * 0.7152 + pixel[2] as f64 * 0.0722
}

fn correlation(image: &RgbImage, dx: u32, dy: u32, mean: f64) -> Option<f64> {
    let mut product = 0.0;
    let mut square_a = 0.0;
    let mut square_b = 0.0;
    let mut samples = 0_u64;
    for y in 0..image.height().saturating_sub(dy) {
        for x in 0..image.width().saturating_sub(dx) {
            let a = image.get_pixel(x, y).0;
            let b = image.get_pixel(x + dx, y + dy).0;
            if !is_water(&a) || !is_water(&b) {
                continue;
            }
            let va = luminance(&a) - mean;
            let vb = luminance(&b) - mean;
            product += va * vb;
            square_a += va * va;
            square_b += vb * vb;
            samples += 1;
        }
    }
    (samples > 1000 && square_a > 0.0 && square_b > 0.0)
        .then(|| product / (square_a * square_b).sqrt())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: analyze_frames <frame-a.png> <frame-b.png>".into());
    }
    let a = image::open(Path::new(&args[1]))?.to_rgb8();
    let b = image::open(Path::new(&args[2]))?.to_rgb8();
    if a.dimensions() != b.dimensions() {
        return Err("frame dimensions differ".into());
    }

    let mut absolute_difference = 0_u64;
    let mut changed = 0_u64;
    let mut water_luminance = 0.0;
    let mut water_pixels = 0_u64;
    for (pixel_a, pixel_b) in a.pixels().zip(b.pixels()) {
        let delta = (0..3)
            .map(|channel| pixel_a[channel].abs_diff(pixel_b[channel]) as u64)
            .sum::<u64>();
        absolute_difference += delta;
        changed += u64::from(delta > 6);
        if is_water(&pixel_a.0) {
            water_luminance += luminance(&pixel_a.0);
            water_pixels += 1;
        }
    }
    let pixels = (a.width() * a.height()) as u64;
    let mean_difference = absolute_difference as f64 / (pixels * 3) as f64;
    let changed_fraction = changed as f64 / pixels as f64;
    let mean_water = water_luminance / water_pixels.max(1) as f64;

    let mut peak_x = (0_u32, f64::NEG_INFINITY);
    let mut peak_y = (0_u32, f64::NEG_INFINITY);
    for lag in 20..=128 {
        if let Some(value) = correlation(&a, lag, 0, mean_water) {
            if value > peak_x.1 {
                peak_x = (lag, value);
            }
        }
        if let Some(value) = correlation(&a, 0, lag, mean_water) {
            if value > peak_y.1 {
                peak_y = (lag, value);
            }
        }
    }
    println!("mean_frame_difference={mean_difference:.4}");
    println!("changed_pixel_fraction={changed_fraction:.4}");
    println!("water_pixels={water_pixels}");
    println!(
        "peak_water_correlation_x=lag:{} value:{:.4}",
        peak_x.0, peak_x.1
    );
    println!(
        "peak_water_correlation_y=lag:{} value:{:.4}",
        peak_y.0, peak_y.1
    );
    Ok(())
}
