use std::fmt::Display;

trait Affichable {
    fn afficher(&self);
}

struct Personne {
    nom: String,
    age: u8,
}

impl Affichable for Personne {
    fn afficher(&self) {
        println!("Personne : nom {} , age {}", self.nom, self.age);
    }
}

struct Livre {
    nom: String,
    author: Personne,
}

impl Affichable for Livre {
    fn afficher(&self) {
        println!("livre  {}", self.nom);
        print!("author is :");
        self.author.afficher();
    }
}

fn affichage<T>(item: T)
where
    T: Affichable,
{
    item.afficher();
}

fn instance_affichable() -> impl Affichable {
    Personne {
        nom: "sidi".to_string(),
        age: 22,
    }
}

// fn test(cond: bool) -> impl Affichable {
//     let p = Personne {
//         nom: "sidi".to_string(),
//         age: 20,
//     };

//     if cond {
//         p
//     } else {
//         Livre {
//             nom: "kh".to_string(),
//             author: p,
//         }
//     }
// }

struct Wrapper<T> {
    val: T,
}

impl<T> Wrapper<T> {
    fn new(val: T) -> Self {
        Wrapper { val: val }
    }
}

impl<T> Wrapper<T>
where
    T: Display,
{
    fn afficher(&self) {
        println!("value {}", self.val);
    }
}

fn main() {
    let p = Personne {
        nom: "sidi".to_string(),
        age: 20,
    };

    // p.afficher();

    let v = Livre {
        nom: "nebil".to_string(),
        author: p,
    };

    v.afficher();

    affichage(instance_affichable());

    // struct :
    let w = Wrapper::new(32);

    w.afficher();

    // let w2 = Wrapper::new(&p);

    // w2.afficher(); // Error
}
