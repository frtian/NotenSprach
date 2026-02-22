use std::collections::HashMap;

pub const GLOBAL_NOTES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

// Usando usize pois serão usados exclusivamente para pular índices
pub const MAJOR_SCALE_INTERVALS: [usize; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const MINOR_SCALE_INTERVALS: [usize; 7] = [2, 1, 2, 2, 1, 2, 2];

/**
Get the notes of a scale based on the root note and the scale intervals.
*/
pub fn get_scale(root: &str, scale_intervals: &[usize]) -> Option<Vec<&'static str>> {
    let mut scale_notes: Vec<&'static str> = Vec::with_capacity(scale_intervals.len() + 1);
    // Busca o índice inicial ou retorna None se a nota for inválida
    let mut root_index = GLOBAL_NOTES.iter().position(|&note| note == root)?;
    // Adiciona a tônica
    scale_notes.push(GLOBAL_NOTES[root_index]);
    // Calcula o resto da escala
    for &interval in scale_intervals {
        root_index += interval;
        scale_notes.push(GLOBAL_NOTES[root_index % 12]);
    }
    Some(scale_notes)
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
        ""   => Ok(key),  // Maior normal     | ex: C
        "m"  => Ok(key),  // Menor normal     | ex: Dm
        "#"  => Ok(key),  // Sustenido        | ex: G#
        "b"  => Ok(key),  // Bemol            | ex: Bb
        "#m" => Ok(key),  // Menor Sustenido  | ex: C#m
        "bm" => Ok(key),  // Menor Bemol      | ex: Ebm
        _    => Err("Invalid Key: Modifiers not recongnized or in a wrong order")
    }
}
