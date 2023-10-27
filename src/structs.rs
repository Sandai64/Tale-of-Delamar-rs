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
    name:      String,
    inventory: PlayerInventory,
    health:    f32,
    position:  String, // Room UUID
}

pub struct PlayerInventory
{
    armor:   Option<Armor>,
    weapon:  Option<Weapon>,
    clothes: Option<Clothes>,
}

pub struct Armor
{
    protection_score: f32,
    durability:       f32,
}

pub struct Weapon
{
    damage_score: f32,
    durability:   f32,
    weapon_type:  WeaponType,
}

pub struct Clothes
{
    style_score: f32,
}

#[non_exhaustive]
pub struct HealthType;
impl HealthType
{
    pub const FOOD:     &str = "Food";
    pub const BAND_AID: &str = "Band-aid";
    pub const POTION:   &str = "Potion";
}

pub struct Health
{
    health_type: HealthType,
    health_score: f32,
}

pub struct Clerk
{
    selling_weapon: Weapon,
    selling_health: Health,
}

pub struct Enemy
{
    damage_value: f32,
}

pub struct Room
{
    enemies: Option<Vec<Enemy>>,
    clerk:   Option<Clerk>,
    previous_room: Option<String>,
    next_room:     String,
}