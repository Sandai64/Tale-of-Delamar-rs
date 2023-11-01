use crate::structs::{Room, Enemy, Clerk, Health, HealthType};

pub fn generate_room(uuid: String) -> Room
{
    return Room
    {
        enemies: Some(vec![
            Enemy { damage_value: 1.25 },
            Enemy { damage_value: 0.25 },
            Enemy { damage_value: 1.14 },
        ]),

        clerk: Some(
            Clerk
            {
                selling_health: Some(
                    Health
                    {
                        health_type: HealthType::POTION,
                        health_score: 2.75
                    }
                ),
                selling_weapon: None
            }
        )
    }
}