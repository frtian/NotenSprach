mod instrument;
mod render;
mod theory;

use crate::theory::chords::*;

fn main() {
  let chord = "F7M";
  let parsed_chord = get_chord(chord);
  println!("Seu acorde: {:?}", parsed_chord);
}
