// Mandelbrot Set in Rust
// Copyright 2013 Ralph Giles, GNU GPLv3

fn escape(cr: float, ci: float) -> uint {
  let mut zr = 0.0;
  let mut zi = 0.0;
  let mut k = 0u;

  while k < 256 {
    let zz = zr*zr - zi*zi + cr;
    zi = 2.0*zr*zi + ci;
    zr = zz;
    //io::println(fmt!("  %u %f %f", k, zr, zi));
    if zr*zr + zi*zi > 4.0 {
      return k;
    }
    k += 1;
  }

  return k;
}

fn main() {
  let mut y = -1.50;
  let d = 3.0 / (64 as float);

  let mut j = 0;
  while j < 64 {
    let mut x = -2.25;
    let mut i = 0;
    while i < 64 {
      let k = escape(x, y);
      //io::println(fmt!("%d %d %f %f %u", i, j, x, y, k));
      if k == 0 {
        io::print(".");
      } else if k < 16 {
        io::print("o");
      } else if k < 32 {
        io::print("x");
      } else if k < 64 {
        io::print("O");
      } else if k < 128 {
        io::print("X");
      } else {
        io::print(" ");
      }
      x += d;
      i += 1;
    }
    io::println("");
    y += d;
    j += 1;
  }

}