#[allow(unused, dead_code, unused_variables, unused_attributes)]

// =============================================================================
//  GLOBAL CONSTANTS
// =============================================================================
pub const GLOBAL_NOTES: [&str; 12] = [
  "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// Usando usize pois serão usados exclusivamente para pular índices
pub const MAJOR_SCALE_INTERVALS: [usize; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const MINOR_SCALE_INTERVALS: [usize; 7] = [2, 1, 2, 2, 1, 2, 2];

// =============================================================================
//  GLOBAL Enums and Structs
// =============================================================================
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Up = 1,
  Down = -1,
}

pub fn validate_key(key: &str) -> Result<&str, &'static str> {
  let char_count = key.chars().count(); // Conta as letras e evita problemas
  // com bytes incompletos
  if char_count == 0 || char_count > 3 {
    return Err("Invalid Key: Incorrect Size (must be 1 to 3 characters)");
  }
  let mut chars = key.chars();
  let root = chars.next().unwrap();
  if !matches!(root, 'A'..='G') {
    return Err("Invalid Key: Must be an uppercase letter from A to G");
  }
  let remaining: String = chars.collect();
  match remaining.as_str() {
    "" => Ok("major"),         // Maior normal     | ex: C
    "m" => Ok("minor"),        // Menor normal     | ex: Dm
    "#" => Ok(key),            // Sustenido        | ex: G#
    "b" => Ok("bemol"),        // Bemol            | ex: Bb
    "#m" => Ok("minor"),       // Menor Sustenido  | ex: C#m
    "bm" => Ok("bemol-minor"), // Menor Bemol      | ex: Ebm
    _ => Err("Invalid Key: Modifiers not recongnized or in a wrong order"),
  }
}

pub fn get_note_index(note: &str) -> usize {
  GLOBAL_NOTES.iter().position(|&n| n == note).unwrap_or(0)
}

pub fn split_root(chord: &str) -> Option<(&'static str, &str)> {
  let mut chars = chord.chars();
  let _root_char = chars.next()?;
  let mut remaining = chars.as_str();

  let mut root_str = &chord[..1]; // Pega a primeira letra como tônica
  let mut is_flat = false;

  if remaining.starts_with('#') {
    root_str = &chord[..2]; // Inclui o sustenido
    remaining = &chord[2..] // remove o que sobrou;
  } else if remaining.starts_with('b') {
    is_flat = true;
    remaining = &chord[2..]; // remove o que sobrou;
  }

  let mut normalized_root = GLOBAL_NOTES
    .iter()
    .find(|&&note| note == root_str)
    .copied()?;

  if is_flat {
    normalized_root = transpose(normalized_root, Direction::Down, 1);
  }

  Some((normalized_root, remaining))
}

/// Transpose a note in the given direction by the number os semitones given.
pub fn transpose(mut key: &str, direction: Direction, steps: usize) -> &str {
  let key_index = get_note_index(key);
  let direction_value = direction as i32;
  let offset = (steps as i32 * direction_value).rem_euclid(12) as usize;
  key = GLOBAL_NOTES[(key_index + offset) % 12];
  key
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_key() {
    assert_eq!(validate_key("C"), Ok("major"));
    assert_eq!(validate_key("Dm"), Ok("minor"));
    assert_eq!(validate_key("G#"), Ok("G#"));
    assert_eq!(validate_key("Bb"), Ok("bemol"));
    assert_eq!(validate_key("C#m"), Ok("minor"));
    assert_eq!(validate_key("Ebm"), Ok("bemol-minor"));
    assert!(validate_key("").is_err());
    assert!(validate_key("H").is_err());
    assert!(validate_key("C##").is_err());
  }

  #[test]
  fn test_get_note_index() {
    assert_eq!(get_note_index("C"), 3);
    assert_eq!(get_note_index("F#"), 9);
    assert_eq!(get_note_index("Bb"), 0);
    assert_eq!(get_note_index("E"), 7);
    assert_eq!(get_note_index("X"), 0); // Not found, returns 0
  }

  #[test]
  fn test_split_root() {
    assert_eq!(split_root("Cmaj7"), Some(("C", "maj7")));
    assert_eq!(split_root("F#m7"), Some(("F#", "m7")));
    assert_eq!(split_root("Bb9"), Some(("A#", "9")));
    assert_eq!(split_root("D"), Some(("D", "")));
    assert_eq!(split_root("#m7"), None);
  }
}
