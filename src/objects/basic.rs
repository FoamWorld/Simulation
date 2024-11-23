pub struct Barrier {
    pub level: f32,
    pub appearence: u8,
}

pub struct Carpet {
    pub resistent: u8,
    pub appearence: u8,
}

pub struct Container {
    pub decoration: String,
    pub refer: Box<dyn Referent>,
}

pub struct Void {}
