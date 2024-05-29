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
    const N: usize = 15;

    pub fn rand() -> Self {
        let data = &DATA[0..Self::N];
        let i = util::rand(Self::N);

        let Self { meaning, choices } = data.get(i).unwrap();
        Self {
            meaning,
            choices: *choices,
        }
    }
}

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
