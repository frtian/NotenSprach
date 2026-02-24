use crate::theory::core::*;
// =============================================================================
// Chords
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Triad {
  Major,
  Minor,
  Augmented,
  Diminished,
  Suspended2,
  Suspended4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Extension {
  Fifth,
  Sixth,
  MajorSeventh,
  MinorSeventh,
  Ninth,
  Eleventh,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
  pub triad: Triad,
  pub extensions: Vec<Extension>,
}

impl Triad {
  pub fn intervals(&self) -> &'static [usize] {
    match self {
      Triad::Major => &[4, 7],
      Triad::Minor => &[3, 7],
      Triad::Augmented => &[4, 8],
      Triad::Diminished => &[3, 6],
      Triad::Suspended2 => &[2, 7],
      Triad::Suspended4 => &[5, 7],
    }
  }
}

impl Extension {
  pub fn interval(&self) -> usize {
    match self {
      Extension::Fifth => 7,
      Extension::Sixth => 9,
      Extension::MajorSeventh => 11,
      Extension::MinorSeventh => 10,
      Extension::Ninth => 14,
      Extension::Eleventh => 17,
    }
  }
}

impl Chord {
  pub fn intervals(&self) -> Vec<usize> {
    let mut result = vec![0];
    result.extend(self.triad.intervals().to_vec());
    result.extend(self.extensions.iter().map(|extension| extension.interval()));
    result
  }
}

pub fn get_chord(chord: &str) -> Option<Vec<&'static str>> {
  let (root_note, remaining) = split_root(chord)?;

  let (extensions, triad_str) = strip_extensions(&remaining);
  let triad = match triad_str {
    "" => Triad::Major,
    "m" => Triad::Minor,
    "aug" => Triad::Augmented,
    "dim" => Triad::Diminished,
    "sus2" => Triad::Suspended2,
    "sus4" => Triad::Suspended4,
    _ => return None, // Modificadores de triade não reconhecidos
  };

  let chord = Chord { triad, extensions };

  let root_index = get_note_index(root_note);
  let intervals = chord.intervals();
  let mut chord_notes = Vec::with_capacity(intervals.len());
  for interval in intervals {
    let note_index = (root_index + interval) % 12;
    chord_notes.push(GLOBAL_NOTES[note_index]);
  }
  Some(chord_notes)
}

pub fn strip_extensions(mut remaing: &str) -> (Vec<Extension>, &str) {
  let mut extensions = Vec::new();

  let patterns: &[(&str, &[Extension])] = &[
    ("7M", &[Extension::MajorSeventh]),
    ("maj7", &[Extension::MajorSeventh]),
    ("m7", &[Extension::MinorSeventh]),
    ("7", &[Extension::MinorSeventh]), // Dominante é tratado como menor sétima
    ("5", &[Extension::Fifth]),
    ("6", &[Extension::Sixth]),
    ("9", &[Extension::Ninth, Extension::MinorSeventh]), // 9 inclui a sétima dominante
    ("11", &[Extension::Eleventh]),
  ];

  loop {
    let mut matched = false;
    for (pattern, exts) in patterns {
      // strip_suffix inverte a string
      if let Some(stripped) = remaing.strip_suffix(pattern) {
        for ext in exts.iter() {
          extensions.push(ext.clone());
        }
        remaing = stripped;
        matched = true;
      }
    }
    if !matched {
      break;
    }
  }
  extensions.reverse();
  (extensions, remaing)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_major_triad_intervals() {
    let triad = Triad::Major;
    assert_eq!(triad.intervals(), &[4, 7]);
  }
  #[test]
  fn test_minor_triad_intervals() {
    let triad = Triad::Minor;
    assert_eq!(triad.intervals(), &[3, 7]);
  }
  #[test]
  fn test_augmented_triad_intervals() {
    let triad = Triad::Augmented;
    assert_eq!(triad.intervals(), &[4, 8]);
  }
  #[test]
  fn test_diminished_triad_intervals() {
    let triad = Triad::Diminished;
    assert_eq!(triad.intervals(), &[3, 6]);
  }
  #[test]
  fn test_suspended2_triad_intervals() {
    let triad = Triad::Suspended2;
    assert_eq!(triad.intervals(), &[2, 7]);
  }
  #[test]
  fn test_suspended4_triad_intervals() {
    let triad = Triad::Suspended4;
    assert_eq!(triad.intervals(), &[5, 7]);
  }
  #[test]
  fn test_major_chord_intervals_with_single_extension() {
    let chord = Chord {
      triad: Triad::Major,
      extensions: vec![Extension::MajorSeventh],
    };
    assert_eq!(chord.intervals(), &[0, 4, 7, 11]);
  }
  #[test]
  fn test_major_chord_intervals_with_multi_extensions() {
    let chord = Chord {
      triad: Triad::Major,
      extensions: vec![
        Extension::MajorSeventh,
        Extension::Ninth,
        Extension::Eleventh,
      ],
    };
    assert_eq!(chord.intervals(), &[0, 4, 7, 11, 14, 17]);
  }
  #[test]
  fn test_stripping_extensions() {
    let (extensions, remaining) = strip_extensions("F#5maj7");
    assert_eq!(extensions, vec![Extension::Fifth, Extension::MajorSeventh]);
    assert_eq!(remaining, "F#");
  }

  #[test]
  fn test_get_chord() {
    assert_eq!(get_chord("C"), Some(vec!["C", "E", "G"]));
    assert_eq!(get_chord("Dm"), Some(vec!["D", "F", "A"]));
    assert_eq!(get_chord("G7"), Some(vec!["G", "B", "D", "F"]));
    assert_eq!(get_chord("F#maj7"), Some(vec!["F#", "A#", "C#", "F"]));
    assert_eq!(get_chord("Bb9"), Some(vec!["A#", "D", "F", "G#", "C"]));
  }
}
