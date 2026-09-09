#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i8)]
pub enum Direction {
  Up = 1,
  Down = -1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub enum Note {
  A = 0,
  ASharp = 1,
  B = 2,
  C = 3,
  CSharp = 4,
  D = 5,
  DSharp = 6,
  E = 7,
  F = 8,
  FSharp = 9,
  G = 10,
  GSharp = 11,
}

impl Note {
  pub fn from_str(note: &str) -> Option<Self> {
    match note {
      "A" => Some(Note::A),
      "A#" | "Bb" => Some(Note::ASharp),
      "B" | "Cb" => Some(Note::B), // Cobrindo casos raros só por segurança
      "C" => Some(Note::C),
      "C#" | "Db" => Some(Note::CSharp),
      "D" => Some(Note::D),
      "D#" | "Eb" => Some(Note::DSharp),
      "E" => Some(Note::E),
      "F" => Some(Note::F),
      "F#" | "Gb" => Some(Note::FSharp),
      "G" => Some(Note::G),
      "G#" | "Ab" => Some(Note::GSharp),
      _ => None,
    }
  }

  pub fn index(self) -> usize {
    self as usize
  }

  pub fn from_index(index: usize) -> Self {
    match index % 12 {
      0 => Note::A,
      1 => Note::ASharp,
      2 => Note::B,
      3 => Note::C,
      4 => Note::CSharp,
      5 => Note::D,
      6 => Note::DSharp,
      7 => Note::E,
      8 => Note::F,
      9 => Note::FSharp,
      10 => Note::G,
      11 => Note::GSharp,
      _ => unreachable!(),
    }
  }

  pub fn transpose(self, direction: Direction, steps: usize) -> Self {
    let direction_value = direction as i32;
    let offset = (steps as i32 * direction_value).rem_euclid(12) as usize;
    let new_index = self.index() + offset;
    Self::from_index(new_index)
  }
}

pub fn split_root(chord: &str) -> Option<(Note, &str)> {
  let mut chars = chord.chars();
  let _first_char = chars.next()?;
  let remaining_chars = chars.as_str();
  let root_len = if remaining_chars.starts_with('#') || remaining_chars.starts_with('b') {
    2 // Se tem acidente, a tônica ocupa 2 caracteres (Ex: "C#")
  } else {
    1 // Se não tem, ocupa 1 caractere (Ex: "C")
  };

  let root_str = &chord[..root_len];
  let modifiers_str = &chord[root_len..];
  let note = Note::from_str(root_str)?;

  Some((note, modifiers_str))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_split_root_note() {
    assert_eq!(split_root("Cmaj7"), Some((Note::C, "maj7")));
    assert_eq!(split_root("F#m7"), Some((Note::FSharp, "m7")));

    assert_eq!(split_root("Bb9"), Some((Note::ASharp, "9")));
    assert_eq!(split_root("Ebm"), Some((Note::DSharp, "m")));

    assert_eq!(split_root("D"), Some((Note::D, "")));

    assert_eq!(split_root(""), None);
    assert_eq!(split_root("#m7"), None);
  }
}
