# Tale of Delamar

```none
  ______      __           ________       __
 /_  __/___ _/ /__  ____  / __/ __ \___  / /___ _____ ___  ____ ______
  / / / __ `/ / _ \/ __ \/ /_/ / / / _ \/ / __ `/ __ `__ \/ __ `/ ___/
 / / / /_/ / /  __/ /_/ / __/ /_/ /  __/ / /_/ / / / / / / /_/ / /
/_/  \__,_/_/\___/\____/_/ /_____/\___/_/\__,_/_/ /_/ /_/\__,_/_/
```

Tale of Delamar is a continuation of an old, abandoned project of mine, of which the source code has been lost to time.

Combining old-school RPG elements & featuring procedurally generated rooms.
While it could be argued that this project has lackluster graphics, the focus here is in learning the inner workings of Rust - and is an exercise in software architecture.

## Focus on RNG

The game is to be centered around procedural generation. Everything takes place in rooms that are identified with a single "*seed*" value,
passed down to a generator function that will output a playable environment.

As such, save data can not only be reduced due to big chunks of information that can now be inferred at runtime (AKA generated *on-the-fly*) - it can also be compressed using compressors [such as zstd](https://facebook.github.io/zstd/).

Thus, a "save file" essentially boils down to saving seed values & differential data to be applied at runtime, when necessary.

## Defined goal

- The goal of the game, is that the player needs to fight enemies in an infinite world generated on-demand, on-the-fly.
- The player gains score points every time an enemy is killed.
- The player shall acquire gear in specific inventory slots defined for that purpose, such as but not limited to :
  - *Armors* (TBD)
  - *Weapons*

  ```rust
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
  ```

  - *Clothes* (TBD)
- Note that only one item of each type can be equipped at the same time.

## Object properties & characteristics

> *Armor*
- Armor is defined with having a "*protection score*", `f32 -> 0..=1`.
  - `0` corresponds to having no protection, thus the player shall receive 100% of the damage values from enemies they might encounter.
  - `1` corresponds to having full protection, thus the player shall receive between 25% to 40% of the damage values.
    - Note: the damage percentage is randomly selected from an inclusive range of `f32 -> 0.25..=0.4`
- Armor shall have a `durability` property. Each hit will decrease the durability property by `1`.
  - If `durability` reaches `0`, the armor is to be considered broken and the player will lose that piece of equipment from their inventory.
- Armor shall be looted from enemies.

> *Weapons*
- Weapons can increase the default damage value the player can deal to enemies - the default value is `1`, it is defined as an `f32 -> 0.25..[inf]`.
- Weapons shall be acquired at shops for gold coins.
- Weapons, like Armor, shall have a durability score.

> *Clothes*
- Clothes have a `style` property, `f32 -> 0..=1`.
  - `0` means the player has no style, and will appear less charismatic to clerks. The clerks will have a tendency to inflate their prices in that situation.
  - `1` means the player has style, and will appear more charismatic. The clerks will then have a tendency to decrease their prices.
- Note: If the player does not change clothes often & goes to the same clerk, the effect will start to reverse.
- Clothes do not have a durability score.

- The player shall talk to clerks in order to buy various items, like Health & Weapons.

## Map & locations

- The map is defined as a hashmap of `Room` objects. Each room object is identified by a UUIDv4, and has the following properties :
  - `enemies: Option<Vec<Enemy>>` : A vector `Enemy` objects
  - `clerks: Option<Clerk>` : A vector of `Clerk` objects
  - `previous_room: Option<String>` : UUIDv4 of the previous room (Optional)
  - `next_room: String` : UUIDv4 of the next room (Required)
- Whilst every room has to have a reference to a next room, there is no hierarchy between them. This alleviates a lot of technical hurdles as there is no concept of graphs to manage anymore.
- The player shall spawn in an initial room, with no `previous_room`
- Progress could also be made regarding procedural name generation, regions (AKA "*clusters*" of rooms), dictated by an initial "universe" seed value.
