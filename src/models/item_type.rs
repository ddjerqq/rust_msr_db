#[derive(Debug)]
pub enum ItemType {
    FishingRod,
    CommonFish,
    RareFish,
    TropicalFish,
    Shark,
    GoldenFish,

    HuntingRifle,
    Pig,
    Deer,
    Bear,
    Wolf,
    Tiger,
    Lion,
    Elephant,

    Shovel,
    CopperCoin,
    Emerald,
    Ruby,
    Sapphire,
    Amethyst,
    Diamond,
    Knife,
}

impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        self.to_i32() == other.to_i32()
    }
}

impl ItemType {
    pub fn from_i32(i: i32) -> Result<Self, String> {
        match i {
            0  => Ok(ItemType::FishingRod),
            1  => Ok(ItemType::CommonFish),
            2  => Ok(ItemType::RareFish),
            3  => Ok(ItemType::TropicalFish),
            4  => Ok(ItemType::Shark),
            5  => Ok(ItemType::GoldenFish),
            6  => Ok(ItemType::HuntingRifle),
            7  => Ok(ItemType::Pig),
            8  => Ok(ItemType::Deer),
            9  => Ok(ItemType::Bear),
            10 => Ok(ItemType::Wolf),
            11 => Ok(ItemType::Tiger),
            12 => Ok(ItemType::Lion),
            13 => Ok(ItemType::Elephant),
            14 => Ok(ItemType::Shovel),
            15 => Ok(ItemType::CopperCoin),
            16 => Ok(ItemType::Emerald),
            17 => Ok(ItemType::Ruby),
            18 => Ok(ItemType::Sapphire),
            19 => Ok(ItemType::Amethyst),
            20 => Ok(ItemType::Diamond),
            21 => Ok(ItemType::Knife),
            _  => Err(format!("Invalid item type: {}", i)),
        }
    }
    
    pub fn to_i32(&self) -> i32 {
        match self {
            ItemType::FishingRod   => 0,
            ItemType::CommonFish   => 1,
            ItemType::RareFish     => 2,
            ItemType::TropicalFish => 3,
            ItemType::Shark        => 4,
            ItemType::GoldenFish   => 5,
            ItemType::HuntingRifle => 6,
            ItemType::Pig          => 7,
            ItemType::Deer         => 8,
            ItemType::Bear         => 9,
            ItemType::Wolf         => 10,
            ItemType::Tiger        => 11,
            ItemType::Lion         => 12,
            ItemType::Elephant     => 13,
            ItemType::Shovel       => 14,
            ItemType::CopperCoin   => 15,
            ItemType::Emerald      => 16,
            ItemType::Ruby         => 17,
            ItemType::Sapphire     => 18,
            ItemType::Amethyst     => 19,
            ItemType::Diamond      => 20,
            ItemType::Knife        => 21,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ItemType::FishingRod   => "Fishing Rod".to_string(),
            ItemType::CommonFish   => "Common Fish".to_string(),
            ItemType::RareFish     => "Rare Fish".to_string(),
            ItemType::TropicalFish => "Tropical Fish".to_string(),
            ItemType::Shark        => "Shark".to_string(),
            ItemType::GoldenFish   => "Golden Fish".to_string(),
            ItemType::HuntingRifle => "Hunting Rifle".to_string(),
            ItemType::Pig          => "Pig".to_string(),
            ItemType::Deer         => "Deer".to_string(),
            ItemType::Bear         => "Bear".to_string(),
            ItemType::Wolf         => "Wolf".to_string(),
            ItemType::Tiger        => "Tiger".to_string(),
            ItemType::Lion         => "Lion".to_string(),
            ItemType::Elephant     => "Elephant".to_string(),
            ItemType::Shovel       => "Shovel".to_string(),
            ItemType::CopperCoin   => "Copper Coin".to_string(),
            ItemType::Emerald      => "Emerald".to_string(),
            ItemType::Ruby         => "Ruby".to_string(),
            ItemType::Sapphire     => "Sapphire".to_string(),
            ItemType::Amethyst     => "Amethyst".to_string(),
            ItemType::Diamond      => "Diamond".to_string(),
            ItemType::Knife        => "Knife".to_string(),
        }
    }
}
