use crate::theory::core::*;

/**
Get the notes of a scale based on the root note and the scale intervals.
*/
pub fn get_scale(key: &str) -> Option<Vec<&'static str>> {
  let mut scale_notes: Vec<&'static str> = Vec::with_capacity(8);
  // check minor or major scale
  let scale = match validate_key(key) {
    Ok("minor") => &MINOR_SCALE_INTERVALS,
    Ok("bemol-minor") => &MINOR_SCALE_INTERVALS,
    _ => &MAJOR_SCALE_INTERVALS,
  };
  let (root, _remaining) = split_root(key)?;
  // Busca o índice inicial ou retorna None se a nota for inválida
  let mut root_index = GLOBAL_NOTES.iter().position(|&note| note == root)?;
  // Adiciona a tônica
  scale_notes.push(GLOBAL_NOTES[root_index]);
  // Calcula o resto da escala
  for &interval in scale {
    root_index += interval;
    scale_notes.push(GLOBAL_NOTES[root_index % 12]);
  }
  Some(scale_notes)
}
