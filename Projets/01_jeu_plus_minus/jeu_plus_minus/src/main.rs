use rand::{self, Rng, rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");
    
    let nombre_secret = rng().random_range(1..101);
    let mut nombre_de_fois: u32 = 0;

    loop {
        nombre_de_fois += 1;
        println!("Veuillez entrer un nombre.");

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée de l'utilisateur.");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins"),
            Ordering::Equal => {
                println!("Vous avez gagné ! avec {} tentatives", nombre_de_fois);
                break;
            }
        }
    }
}
