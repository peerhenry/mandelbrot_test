extern crate num;
use num::complex::Complex;
extern crate image;
use std::fs::File;
use std::path::Path;
extern crate time;
use time::PreciseTime;

const WIDTH: u32 = 1920*2;
const HEIGHT: u32 = 1080*2;
const LIMIT: u8 = 255; // maximum number of iterations per pixel

// full image
const LEFT: f32 = -2.5;
const RIGHT: f32 = 0.5;
const TOP: f32 = 1.0;
const BOTTOM: f32 = -1.0;

// const LEFT: f32 = -0.35;
// const RIGHT: f32 = 0.15;
// const TOP: f32 = 1.0;
// const BOTTOM: f32 = 0.7;

fn get_color(c: Complex<f32>) -> image::Luma<u8>{
  let mut next_c = c;
  let mut count = LIMIT; // do max 100 steps

  while count > 0 {
    if next_c.norm_sqr() > 4.0 {
      break;
    }
    next_c = next_c*next_c + c;
    count = count - 1;
  }

  let frac = count as f64/LIMIT as f64;
  //let number = frac*255.0 + 1.0;
  //let logval = number.log(256.0); // log base 256
  let val: u8 = (frac*frac*frac*frac*255.0) as u8;
  image::Luma([val])
}

fn main(){
  let start = PreciseTime::now();
  println!("Starting Mandelbrot image generation...");
  let v_range = TOP - BOTTOM;
  let h_range = RIGHT - LEFT;
  let img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |i, j| {
    let x = ( (i as f32/WIDTH as f32)  )*h_range + LEFT;
    let y = ( (j as f32/HEIGHT as f32) )*v_range + BOTTOM;
    let c = Complex{re: x, im: y};
    get_color(c)
  });

  let ref mut fout = File::create(&Path::new("images/mandelbrot_test.png")).unwrap();
  let _ = image::ImageLuma8(img).save(fout, image::PNG);
  let end = PreciseTime::now();
  println!("Program finished in {} seconds.", start.to(end));
}