pub enum Resolution {
    QHD,
    FHD
}

impl Resolution {
    pub fn value(&self) -> [i32;2] {
        match *self {
            Resolution::QHD => {
                [3216, 1440]
            },
            Resolution::FHD => {
                [2412, 1080]
            }
        }
    }
}
