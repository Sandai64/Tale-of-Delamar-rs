use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use uuid::Uuid;

use crate::structs;
use crate::consts;

pub fn generate_room(room_seed: String) -> structs::Room
{
    // Seed RNG
    let mut rng: Pcg64 = Seeder::from(room_seed).make_rng();

    // Enemy generation
    let mut room_enemies: Option<Vec<structs::Enemy>> = None;
    if rng.gen_bool(consts::ROOM_CHANCE_INCLUDES_ENEMIES)
    {
        if let Some(room_enemies_inner) = room_enemies.as_mut()
        {
            for _ in 0..=rng.gen_range(1..=3)
            {
                room_enemies_inner.push( structs::Enemy { damage_value: rng.gen_range(0.0..=consts::ENEMY_MAX_DAMAGE_VALUE) } );
            }
        }
    }

    return structs::Room
    {
        enemies: room_enemies,
        clerk: Some(
            structs::Clerk
            {
                selling_health: Some(
                    structs::Health
                    {
                        health_type: structs::HealthType::POTION,
                        health_score: 2.75
                    }
                ),
                selling_weapon: None
            }
        ),
        previous_room: None,
        next_room: Uuid::new_v4().to_string()
    }
}
