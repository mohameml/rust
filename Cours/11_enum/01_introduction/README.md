# Cour : **Enum**

## 1. **Introduction aux `enum` en Rust**

-   **Définition :**

    > Un `enum` (_enumeration_) permet de définir un **type qui peut être une de plusieurs variantes**. Chaque variante peut éventuellement contenir des données.

-   **Syntaxe de base:**

    ```rust
    enum Direction {
        North,
        South,
        East,
        West,
    }
    ```

-   **Exemple :**

    ```rust
    enum Direction {
        North,
        South,
        East,
        West,
    }

    fn dir_message(direction: Direction) {
        match direction {
            Direction::North => println!("Vous allez au nord."),
            Direction::South => println!("Vous allez au sud."),
            Direction::East  => println!("Vous allez à l’est."),
            Direction::West  => println!("Vous allez à l’ouest."),
        }
    }

    fn main() {
        let d = Direction::East;
        dir_message(d);
    }
    ```

## 2. **Enum avec données en Rust:**

-   **Définition:**

    > En Rust, une énumération (enum) avec données est un type de donnée qui permet de définir un ensemble de variantes, où chaque variante peut optionnellement contenir des données associées. C'est une fonctionnalité puissante qui combine la notion d'énumération avec la capacité de porter des données, similaire aux unions taguées dans d'autres langages.

-   **Syntaxe:**

    La syntaxe de base pour définir une enum avec données est :

    ```rust
    enum NomEnum {
        Variante1(TypeDonnee),
        Variante2(TypeDonnee1, TypeDonnee2),
        Variante3 { champ1: Type1, champ2: Type2 },
        VarianteSansDonnee,
    }
    ```

    Les variantes peuvent contenir :

    -   Des données positionnelles (comme un tuple)
    -   Des données nommées (comme une structure)
    -   Aucune donnée

-   **Exemple 1 : Enum simple avec données**

    ```rust
    enum Message {
        Quitter,
        Deplacer { x: i32, y: i32 },
        Ecrire(String),
        ChangerCouleur(i32, i32, i32),
    }

    // Utilisation
    let msg1 = Message::Quitter;
    let msg2 = Message::Deplacer { x: 10, y: 20 };
    let msg3 = Message::Ecrire(String::from("Bonjour"));
    let msg4 = Message::ChangerCouleur(255, 0, 0);
    ```

-   **Exemple 2 : Gestion des erreurs**

    ```rust
    enum ResultatCalcul {
        Succes(i32),
        Echec(String),
    }

    fn diviser(a: i32, b: i32) -> ResultatCalcul {
        if b == 0 {
            ResultatCalcul::Echec(String::from("Division par zéro"))
        } else {
            ResultatCalcul::Succes(a / b)
        }
    }

    // Utilisation
    match diviser(10, 2) {
        ResultatCalcul::Succes(resultat) => println!("Résultat: {}", resultat),
        ResultatCalcul::Echec(erreur) => println!("Erreur: {}", erreur),
    }
    ```

-   **Exemple 3 : Arbre d'expressions**

    ```rust
    enum Expression {
        Nombre(i32),
        Addition(Box<Expression>, Box<Expression>),
        Soustraction(Box<Expression>, Box<Expression>),
    }

    // Création d'une expression: (5 + 3) - 2
    let expr = Expression::Soustraction(
        Box::new(Expression::Addition(
            Box::new(Expression::Nombre(5)),
            Box::new(Expression::Nombre(3)),
        )),
        Box::new(Expression::Nombre(2)),
    );

    // Fonction d'évaluation
    fn evaluer(expr: Expression) -> i32 {
        match expr {
            Expression::Nombre(n) => n,
            Expression::Addition(gauche, droite) => evaluer(*gauche) + evaluer(*droite),
            Expression::Soustraction(gauche, droite) => evaluer(*gauche) - evaluer(*droite),
        }
    }

    println!("Résultat: {}", evaluer(expr)); // Affiche 6
    ```

## 3. Méthodes sur les `enum` avec `impl`

-   **Définition:**

    > En Rust, on peut définir des méthodes pour les enums en utilisant un bloc `impl`, exactement comme pour les structs. Cela permet d'encapsuler la logique associée à l'enum et ses variantes.

-   **Syntaxe de base:**

    ```rust
    impl NomEnum {
        fn nom_methode(&self, parametres) -> TypeRetour {
            // Corps de la méthode
        }
    }
    ```

-   **Différence avec les structs**

    -   La seule différence entre implémenter des méthodes pour un enum et un struct est que pour un enum, vous devez généralement utiliser le pattern matching dans le corps des méthodes pour gérer les différentes variantes.

    -   Cette fonctionnalité rend les enums en Rust extrêmement puissants, permettant de combiner les avantages des enums avec ceux de la programmation orientée objet.

-   **Exemple 1 : Méthode simple**

    ```rust
    enum Status {
        Connected,
        Disconnected,
        Error(String),
    }

    impl Status {
        // Méthode qui vérifie si connecté
        fn is_connected(&self) -> bool {
            match self {
                Status::Connected => true,
                _ => false,
            }
        }

        // Méthode qui retourne le message d'erreur si disponible
        fn error_message(&self) -> Option<&str> {
            match self {
                Status::Error(msg) => Some(msg),
                _ => None,
            }
        }
    }

    // Utilisation
    let status = Status::Error("Timeout".to_string());
    println!("Connecté: {}", status.is_connected()); // false
    println!("Message: {:?}", status.error_message()); // Some("Timeout")
    ```

-   **Exemple 2:**

    ```rust
    enum Operation {
        Add,
        Subtract,
    }

    impl Operation {
        fn apply(&self, a: i32, b: i32) -> i32 {
            match self {
                Operation::Add => a + b,
                Operation::Subtract => a - b,
            }
        }
    }

    fn main() {
        let op = Operation::Add;
        println!("Résultat: {}", op.apply(5, 3)); // Résultat: 8
    }
    ```

-   **Exemple 3 : Méthodes avancées avec pattern matching**

    ```rust
    enum Forme {
        Cercle { rayon: f64 },
        Rectangle { largeur: f64, hauteur: f64 },
    }

    impl Forme {
        fn aire(&self) -> f64 {
            match self {
                Forme::Cercle { rayon } => std::f64::consts::PI * rayon * rayon,
                Forme::Rectangle { largeur, hauteur } => largeur * hauteur,
            }
        }

        fn agrandir(&mut self, facteur: f64) {
            match self {
                Forme::Cercle { rayon } => *rayon *= facteur,
                Forme::Rectangle { largeur, hauteur } => {
                    *largeur *= facteur;
                    *hauteur *= facteur;
                }
            }
        }
    }

    // Utilisation
    let mut cercle = Forme::Cercle { rayon: 2.0 };
    println!("Aire initiale: {}", cercle.aire()); // ~12.566
    cercle.agrandir(1.5);
    println!("Aire agrandie: {}", cercle.aire()); // ~28.274
    ```
