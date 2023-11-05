use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use uuid::Uuid;

// Local Game libs
mod structs;
mod io;
mod map;
mod consts;

fn main()
{
    let mut rng: Pcg64 = Seeder::from(Uuid::new_v4().to_string()).make_rng();
    println!("{}", rng.gen::<u8>().to_string());

    println!("---- Game ----");
    let player_name: String = io::stdin_input("Enter your name : ", Some(true));

    println!("---- Start ----");



}
