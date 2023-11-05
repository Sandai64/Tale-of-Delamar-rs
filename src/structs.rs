#![allow(non_camel_case_types)]

// -----------------------------
// ENUMS
// -----------------------------

pub enum WeaponType
{
    SHOVEL,
    BASEBALL_BAT,
    SWORD,
    AMERICAN_KNUCKLES,
    COOKING_PAN,
    CHAIR,
    AXE,
    TELESCOPIC_BATON,
    STATS_TEXTBOOK,
    DRUM_STICKS,
}

pub enum HealthType
{
    POTION,
    BAND_AID,
    FOOD,
}

// -----------------------------
// STRUCTS
// -----------------------------

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
    pub enemies:       Option<Vec<Enemy>>,
    pub clerk:         Option<Clerk>,
    pub previous_room: Option<String>,
    pub next_room:     String,
}
