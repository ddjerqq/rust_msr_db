use rand::Rng;

#[derive(Debug)]
pub enum Rarity {
    FactoryNew(f64),
    MinimalWear(f64),
    FieldTested(f64),
    WellWorn(f64),
    BattleScarred(f64),
}

impl Rarity {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self::from_f64(rng.gen::<f64>())
    }

    pub fn from_f64(value: f64) -> Rarity {
        if (0.0..=0.07).contains(&value) {
            Rarity::FactoryNew(value)
        } else if (0.07..=0.15).contains(&value) {
            Rarity::MinimalWear(value)
        } else if (0.15..=0.38).contains(&value) {
            Rarity::FieldTested(value)
        } else if (0.38..=0.45).contains(&value) {
            Rarity::WellWorn(value)
        } else {
            Rarity::BattleScarred(value)
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Rarity::FactoryNew(value) => *value,
            Rarity::MinimalWear(value) => *value,
            Rarity::FieldTested(value) => *value,
            Rarity::WellWorn(value) => *value,
            Rarity::BattleScarred(value) => *value,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Rarity::FactoryNew(_) => "Factory New",
            Rarity::MinimalWear(_) => "Minimal Wear",
            Rarity::FieldTested(_) => "Field-Tested",
            Rarity::WellWorn(_) => "Well-Worn",
            Rarity::BattleScarred(_) => "Battle-Scarred",
        }
    }
}
