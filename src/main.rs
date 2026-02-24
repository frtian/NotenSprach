mod instrument;
mod render;
mod theory;

use crate::theory::chords::*;

fn main() {
  let pattern = "F#5maj7";
  let (extentions, remaining) = strip_extensions(pattern);

  println!("Extensions: {:?}\n remaining: {:?}", extentions, remaining);
}
