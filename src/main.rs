use std::io::{stderr, Write};

fn main() {
  const IMAGE_WIDTH: u64 = 256;
  const IMAGE_HEIGHT: u64 = 256;

  println!("P3");
  println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
  println!("255");

  for j in (0..IMAGE_HEIGHT).rev() {
    eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
    stderr().flush().unwrap();
    for i in 0..IMAGE_WIDTH {
      println!("{} {} {}", j, i, 63);
    }
  }
  
}