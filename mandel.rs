// Mandelbrot Set in Rust
// Copyright 2014 Ralph Giles, GNU GPLv3

struct Complex {
  r: f64,
  i: f64
}

fn escape(c: Complex) -> i32 {
  let max = 256;
  let mut z = Complex { r: 0.0, i: 0.0 };
  for iter in range(0, max) {
    if z.r * z.r + z.i * z.i > 4.0 {
      return iter;
    }
    let zr = z.r * z.r - z.i * z.i + c.r;
    z.i = 2.0 * z.r * z.i + c.i;
    z.r = zr;
  }
  return max;
}

fn main() {
  println!("Hello Benoit!");
  let irange = 40;
  let jrange = 20;
  let dr = 3.0 / irange as f64;
  let di = 3.0 / jrange as f64;
  for j in range(0, jrange) {
    let ci = di * j as f64 - 1.5;
    for i in range(0, irange) {
      let cr = dr * i as f64 - 2.25;
      let e = escape(Complex { r: cr, i: ci });
      if e > 255 {
        print!("*");
      } else {
        print!(" ");
      }
    }
    println!("");
  }
}
