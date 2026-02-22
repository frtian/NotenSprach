mod modules;
use crate::modules::common::*;

fn main() {
    let note = "1233";
    let scale_prev = get_scale(note, &MAJOR_SCALE_INTERVALS);
    println!("Resultado: {:?}", scale_prev);
    let my_key = validate_key("J#");
    // let my_key = if validate_key("C#") { "válido" } else { "inválido" };
    println!("Seu tom é: {:?}", my_key)
}
