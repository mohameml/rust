# Cour : **Struct**

## 1. **Introduction aux `struct` en Rust:**

-   **Définition :**

    > Une `struct` (abréviation de _structure_) est un type de données personnalisées qui vous permet de regrouper plusieurs variables nommées sous une seule entité logique.

-   **Syntaxe :**

    ```rust
    struct NomDeLaStruct {
        champ1: Type1,
        champ2: Type2,
    }
    ```

-   **Exemple :**

    ```rust
    struct Person {
        name: String,
        age: u8,
    }

    fn main() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
        };
        println!("{} a {} ans", person.name, person.age);
    }
    ```

## 2. **Implémentation d’une `struct` avec `impl`**

-   **Définition :**

    > Le mot-clé `impl` permet d’implémenter des **fonctions associées** à une struct, comme des méthodes ou des constructeurs personnalisés.

-   **Syntaxe :**

    ```rust
    impl NomDeLaStruct {
        fn nom_de_methode(&self) -> TypeRetour {
            // corps de la méthode
        }

        fn nouvelle_instance(...) -> NomDeLaStruct {
            NomDeLaStruct {
                // initialisation
            }
        }
    }
    ```

-   **Exemple :**

    ```rust
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

    fn main() {
        let rect = Rectangle::new(10, 5);
        println!("Aire: {}", rect.area());
    }
    ```

## 3. **Tuple Structs:**

-   **Définition :**

    > Une `tuple struct` ressemble à un tuple, mais elle a un **nom de type**. Elle est utile lorsque vous voulez nommer un tuple pour plus de clarté.

-   **Syntaxe :**

    ```rust
    struct NomStruct(Type1, Type2, ...);
    ```

-   **Exemple :**

    ```rust
    struct Color(u8, u8, u8); // RGB
    struct Point(i32, i32);

    fn main() {
        let red = Color(255, 0, 0);
        let origin = Point(0, 0);

        println!("Red: {}, {}, {}", red.0, red.1, red.2);
        println!("Point origine: ({}, {})", origin.0, origin.1);
    }
    ```

## 4. **Méthodes et méthodes associées:**

-   **Méthodes :**

    > Les méthodes sont des fonctions définies **dans un bloc `impl`** et qui **prennent `self` en paramètre** (ou `&self`, `&mut self`). Elles permettent d’agir sur une instance.

-   **Méthodes associées :**

    > Ce sont des fonctions définies avec `impl` qui **ne prennent pas `self`**. Elles sont souvent utilisées comme constructeurs.

-   **Syntaxe :**

    ```rust
    impl NomStruct {
        // Méthode
        fn afficher(&self) {
            println!("Valeur: {}", self.valeur);
        }

        // Méthode associée
        fn nouvelle(valeur: i32) -> Self {
            Self { valeur }
        }
    }
    ```

-   **Exemple :**

    ```rust
    struct Compteur {
        valeur: i32,
    }

    impl Compteur {
        fn increment(&mut self) {
            self.valeur += 1;
        }

        fn afficher(&self) {
            println!("Valeur: {}", self.valeur);
        }

        fn nouveau() -> Self {
            Self { valeur: 0 }
        }
    }

    fn main() {
        let mut c = Compteur::nouveau();
        c.increment();
        c.afficher(); // Affiche : Valeur: 1
    }
    ```
