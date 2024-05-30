use super::util;

#[derive(Debug)]
pub struct Data {
    meaning: &'static str,
    choices: [&'static str; 3],
}

impl Data {
    const fn new() -> Self {
        Self {
            meaning: "",
            choices: ["", "", ""],
        }
    }

    pub fn meaning(&self) -> &str {
        self.meaning
    }

    pub fn choices(&self) -> &[&str] {
        &self.choices
    }
}

impl Data {
    const FILL: usize = 15;
    pub const DATA: [Data; 100] = [
        Data {
            meaning: "Umbrella",
            choices: ["Payung", "Jas Hujan", "Mantel"],
        },
        Data {
            meaning: "Computer",
            choices: ["Komputer", "Papan ketik", "Telepon"],
        },
        Data {
            meaning: "Book",
            choices: ["Buku", "Pensil", "Penghapus"],
        },
        Data {
            meaning: "Window",
            choices: ["Jendela", "Pintu", "Rumah"],
        },
        Data {
            meaning: "Hospital",
            choices: ["Rumah sakit", "Rumah", "Hotel"],
        },
        Data {
            meaning: "Tree",
            choices: ["Pohon", "Kayu", "Tumbuhan"],
        },
        Data {
            meaning: "Summer",
            choices: ["Pantai", "Gurun", "Hutan"],
        },
        Data {
            meaning: "Rain",
            choices: ["Hujan", "Panas", "Mendung"],
        },
        Data {
            meaning: "Bicycle",
            choices: ["Sepeda", "Motor", "Mobil"],
        },
        Data {
            meaning: "Sport",
            choices: ["Olahraga", "Bersepeda", "Naik kendaraan"],
        },
        Data {
            meaning: "Reading",
            choices: ["Membaca", "Menulis", "Melihat"],
        },
        Data {
            meaning: "Painting",
            choices: ["Lukisan", "Gambar", "Tulisan"],
        },
        Data {
            meaning: "Rainbow",
            choices: ["Pelangi", "Hujan", "Awan"],
        },
        Data {
            meaning: "Cloud",
            choices: ["Awan", "Mendung", "Angin"],
        },
        Data {
            meaning: "Highway",
            choices: ["Jalan raya", "Sawah", "Pedesaan"],
        },
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
        Data::new(),
    ];

    pub fn rand() -> Self {
        let data = &Self::DATA[0..Self::FILL];
        let i = util::rand(Self::FILL);

        let Self { meaning, choices } = data.get(i).unwrap();
        Self {
            meaning: *meaning,
            choices: *choices,
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::Data;

    #[wasm_bindgen_test]
    fn new_data() {
        let data = Data::new();
        assert!(data.meaning().is_empty());
        assert_eq!(data.choices().len(), 3);
        assert_eq!(data.choices(), ["", "", ""]);
    }

    #[wasm_bindgen_test]
    fn current_fill_data() {
        let fill = Data::FILL;
        assert_eq!(fill, 15)
    }

    #[wasm_bindgen_test]
    fn rand_data() {
        let data = Data::rand();
        assert_eq!(data.choices().len(), 3);
        data.choices()
            .iter()
            .for_each(|choice| assert!(!choice.is_empty()));

        assert!(!data.meaning().is_empty());
    }

    #[wasm_bindgen_test]
    fn current_data() {
        let data = Data::DATA;
        assert_eq!(data.len(), 100);

        for (i, data) in data.iter().enumerate() {
            if i < Data::FILL {
                assert!(!data.meaning.is_empty());
                data.choices()
                    .iter()
                    .for_each(|choice| assert!(!choice.is_empty()))
            } else {
                assert!(data.meaning.is_empty());
                data.choices()
                    .iter()
                    .for_each(|choice| assert!(choice.is_empty()))
            }
        }
    }
}
