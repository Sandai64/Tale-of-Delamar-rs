// STRUCTS

#[non_exhaustive]
pub struct WeaponType;
impl WeaponType
{
    pub const SHOVEL:            &str = "Shovel";
    pub const BASEBALL_BAT:      &str = "Baseball Bat";
    pub const SWORD:             &str = "Sword";
    pub const AMERICAN_KNUCKLES: &str = "American Knuckles";
    pub const COOKING_PAN:       &str = "Cooking Pan";
    pub const CHAIR:             &str = "Chair";
    pub const AXE:               &str = "Axe";
    pub const TELESCOPIC_BATON:  &str = "Telescopic Baton";
    pub const STATS_TEXTBOOK:    &str = "Stats Textbook";
    pub const DRUM_STICKS:       &str = "Drum Sticks";
}


pub struct Player
{
    pub name:      String,
    pub inventory: PlayerInventory,
    pub health:    f32,
    pub position:  String, // Room UUID
}

pub struct PlayerInventory
{
    pub armor:   Option<Armor>,
    pub weapon:  Option<Weapon>,
    pub clothes: Option<Clothes>,
}

pub struct Armor
{
    pub protection_score: f32,
    pub durability:       f32,
}

pub struct Weapon
{
    pub damage_score: f32,
    pub durability:   f32,
    pub weapon_type:  WeaponType,
}

pub struct Clothes
{
    pub style_score: f32,
}


enum HealthType
{
    POTION,
    BAND_AID,
    FOOD,
}

// #[non_exhaustive]
// pub struct HealthType;
// impl HealthType
// {
//     pub const FOOD:     &str = "Food";
//     pub const BAND_AID: &str = "Band-aid";
//     pub const POTION:   &str = "Potion";
// }

pub struct Health
{
    pub health_type: HealthType,
    pub health_score: f32,
}

pub struct Clerk
{
    pub selling_weapon: Option<Weapon>,
    pub selling_health: Option<Health>,
}

pub struct Enemy
{
    pub damage_value: f32,
}

pub struct Room
{
    pub enemies: Option<Vec<Enemy>>,
    pub clerk:   Option<Clerk>,
    pub previous_room: Option<String>,
    pub next_room:     String,
}