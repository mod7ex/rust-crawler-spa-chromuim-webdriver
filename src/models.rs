#[derive(Debug)]
pub struct Flat {
    pub apartment: u8,
    pub rooms: String,
    // pub price: u16,
    pub area: String,
    pub plan: String,
}

#[derive(Debug)]
pub struct Building {
    pub id: u8,
    pub name: String,
    pub flats: Vec<Flat>
}

#[derive(Debug)]
pub struct Complex {
    pub id: u8,
    pub name: String,
    pub buildings: Vec<Building>
}

impl Complex {
    pub fn new(name: String) -> Self {
        Complex {
            id: 10,
            name,
            buildings: Vec::new()
        }
    }
}

impl Building {
    pub fn new(name: String) -> Self {
        Building {
            id: 10,
            name,
            flats: Vec::new()
        }
    }
}