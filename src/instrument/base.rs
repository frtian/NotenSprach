#[allow(dead_code, unused)]
#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub strings: Vec<&'static str>,
    pub fret_count: usize,
}

impl Instrument {
    pub fn standard_guitar() -> Self {
        Self {
            name: String::from("Standard Guitar"),
            strings: vec!["E", "A", "D", "G", "B", "E"],
            fret_count: 22,
        }
    }
    pub fn drop_d() -> Self {
        Self {
            name: String::from("Drop D Guitar"),
            strings: vec!["B", "A", "D", "G", "B", "E"],
            fret_count: 22,
        }
    }
    pub fn bass() -> Self {
        Self {
            name: String::from("Standard Bass"),
            strings: vec!["E", "A", "D", "G"],
            fret_count: 20,
        }
    }
    pub fn ukulele() -> Self {
        Self {
            name: String::from("Standard Guitar"),
            strings: vec!["G", "C", "E", "A"],
            fret_count: 14,
        }
    }
}
