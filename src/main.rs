//! # The Sierpinski Triangle
//!

extern crate image;
extern crate rand;

use rand::distributions::IndependentSample;
use std::env;
use std::fs::File;
use std::path::Path;

///
/// Generate the sierpinski triangle image
///
/// # Arguments
/// * `filename` - Path to the file to save image in
/// * `n` - Amount of iterations for the "Chaos Game" method of generation of
/// triangle
///
fn generate_sierpinski_tri<'a>(filename: &'a str,
                               img_width: u32,
                               img_height: u32,
                               n: u32,
                               clr: [u8; 4]) {
  use rand::distributions::Range as RandRange;
  use image::{ImageBuffer, ImageFormat, ImageRgba8, Rgba};
  use rand::thread_rng;

  // Create a new random number generator and some amount of random distributions
  let mut rng = thread_rng();
  let rand_starting_pnt_x_range = RandRange::new(0, img_width);
  let rand_starting_pnt_y_range = RandRange::new(0, img_height);
  let rand_corner_range = RandRange::new(0, 3);

  // Pick three corners
  let corners = [(0, img_height), (img_width, img_height), ((img_width / 2) as u32, 0)];

  // Select random starting point
  let mut pnt = (rand_starting_pnt_x_range.ind_sample(&mut rng),
                 rand_starting_pnt_y_range.ind_sample(&mut rng));

  // Create a new image buffer
  let mut img_buf: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(img_width, img_height);

  // Create a new pixel to use
  let pixel = Rgba(clr);

  // Generate the image
  for i in 1..n {
    // Write a pixel
    img_buf.put_pixel(pnt.0, pnt.1, pixel);

    // Select random corner
    let rand_corner_num = rand_corner_range.ind_sample(&mut rng);
    let rand_corner = corners[rand_corner_num];

    // Calculate the position of the next point
    pnt.0 = (pnt.0 + rand_corner.0) / 2;
    pnt.1 = (pnt.1 + rand_corner.1) / 2;
  }

  // Open a new file and save the image
  let path = Path::new(filename);
  let mut file = File::create(path).unwrap();
  let _ = ImageRgba8(img_buf).save(&mut file, ImageFormat::PNG).unwrap();
}

///
/// Main function
///
fn main() {
  let args: Vec<_> = env::args().collect();
  // Get the amount of iterations
  if args.len() < 2 {
    println!("[!] Error: you should give the number of iterations");
  }
  let n = args[1].parse::<i32>().unwrap();
  if n <= 0 {
    println!("[!] Error: you cannot get a Sierpinski Triangle with amount of iterations <= 0");
    return;
  }

  // Generate the actual triangle
  generate_sierpinski_tri("triangle.png", 1024, 1024, n as u32, [255, 0, 0, 255]);
}
