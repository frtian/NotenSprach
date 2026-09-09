#![allow(unused)]
use crate::theory::core::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ScaleType {
  Major,
  NaturalMinor,
}

impl ScaleType {
  pub fn step_sizes(&self) -> &'static [usize] {
    match self {
      ScaleType::Major => &[2, 2, 1, 2, 2, 2, 1],
      ScaleType::NaturalMinor => &[2, 1, 2, 2, 1, 2, 2],
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scale {
  pub root: Note,
  pub scale_type: ScaleType,
}

impl Scale {
  pub fn notes(&self) -> Vec<Note> {
    let steps = self.scale_type.step_sizes();
    let mut result = Vec::with_capacity(steps.len() + 1);

    let mut current_note = self.root;
    result.push(current_note);

    for &step in steps {
      current_note = current_note.transpose(Direction::Up, step);
      result.push(current_note);
    }

    result
  }
}

pub fn get_scale(key: &str) -> Option<Vec<Note>> {
  let (root, modifiers) = split_root(key)?;

  // 3. Descobrimos o tipo de escala baseado no que sobrou
  let scale_type = match modifiers {
    "" => ScaleType::Major,
    "m" => ScaleType::NaturalMinor,
    _ => return None, // Se passarem "Cdim" ou lixo textual, negamos.
  };

  // 4. Instanciamos a escala e geramos as notas
  let scale = Scale { root, scale_type };
  Some(scale.notes())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_major_scale_generation() {
    let scale = get_scale("C").unwrap();
    let expected = vec![
      Note::C,
      Note::D,
      Note::E,
      Note::F,
      Note::G,
      Note::A,
      Note::B,
      Note::C,
    ];
    assert_eq!(scale, expected);
  }

  #[test]
  fn test_flat_minor_scale() {
    let scale = get_scale("Ebm").unwrap();

    let expected = vec![
      Note::DSharp,
      Note::F,
      Note::FSharp,
      Note::GSharp,
      Note::ASharp,
      Note::B,
      Note::CSharp,
      Note::DSharp,
    ];
    assert_eq!(scale, expected);
  }
}
