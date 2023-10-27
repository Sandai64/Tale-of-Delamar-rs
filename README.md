# Game

## Defined goal

- The goal of the game, is that the player needs to fight enemies in an infinite world generated on-demand, on-the-fly.
- The player shall acquire gear in specific inventory slots defined for that purpose, such as but not limited to :
  - *Armor*
  - *Weapon*
  - *Clothes*
- Note that only one item of each type can be equipped at the same time.

## Object properties & characteristics

> *Armor*
- Armor is defined with having a "*protection score*", ranging from `0` to `1`.
  - `0` corresponds to having no protection, thus the player shall receive 100% of the damage values from enemies they might encounter.
  - `1` corresponds to having full protection, thus the player shall receive between 25% to 40% of the damage values.
    - Note: the damage percentage is randomly selected from an inclusive range of `f32 -> 0.25..=0.4`
- Armor shall have a `durability` property. Each hit will decrease the durability property by `1`.
  - If `durability` reaches `0`, the armor is to be considered broken and the player will lose that piece of equippement from their inventory.
- Armor shall be looted from enemies.

> *Weapons*
- Weapons can increase the default damage value the player can deal to enemies - the default value is `1`.
- Weapons shall be acquired at shops for gold coins.
- Weapons, like Armor, shall have a durability score.

> *Clothes*
- Clothes have a `style` property, ranging from `0` to `1`.
  - `0` means the player has no style, and will appear less charismatic to clerks. The clerks will have a tendency to inflate their prices in that situation.
  - `1` means the player has style, and will appear more charismatic. The clerks will then have a tendency to decrease their prices.
- Note: If the player does not change clothes often & goes to the same clerk, the effect will start to reverse.
- Clothes do not have a durability score.

- The player shall talk to clerks in order to buy various items, like Health & Weapons.

## Map & locations

- The map is defined as a hashmap of `Room` objects. Each room object is identified by a UUID, and has the following properties :
  - `enemies` : An array of `Enemy` objects
  - `clerks`: An array of `Clerk` objects
  - `previous_room`: UUID of the previous room
  - `next_room` UUID of the next room
- The player shall spawn in an initial room, with no `previous_room`