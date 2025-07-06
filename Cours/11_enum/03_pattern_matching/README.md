# Cour : **Pattern Matching:**

## 1. **Pattern Matching avec les enums:**

-   **Définition:**

    Le pattern matching (ou "filtrage par motif") est un mécanisme puissant de Rust qui permet:

    -   **D'analyser** une valeur enum
    -   **D'extraire** ses données internes
    -   **D'exécuter** du code différent selon la variante

-   **Syntaxe de base:**

    ```rust
    match expression {
        Motif1 => code1,
        Motif2 => code2,
        // ...
        _ => code_par_defaut
    }
    ```

-   **Exemple : Enum avec données**

    ```rust
    enum Message {
        Quitter,
        Deplacer { x: i32, y: i32 },
        Ecrire(String),
        ChangerCouleur(u8, u8, u8),
    }

    fn traiter_message(msg: Message) {
        match msg {
            Message::Quitter => println!("L'application va s'arrêter"),
            Message::Deplacer { x, y } => {
                println!("Déplacement vers ({}, {})", x, y)
            },
            Message::Ecrire(texte) => println!("Message reçu: {}", texte),
            Message::ChangerCouleur(r, g, b) => {
                println!("Nouvelle couleur: R{} G{} B{}", r, g, b)
            },
        }
    }
    ```

## 2. **if let:**

-   **Définition:**

    > `if let` est une syntaxe concise qui combine **`if`** et **`let`** pour faire correspondre un seul motif (_pattern_) tout en ignorant les autres cas. C'est très utile pour simplifier le traitement des `Option`, `Result` ou des `enum` quand on ne s'intéresse qu'à un seul cas.

-   **Syntaxe de base**

    ```rust
    if let Motif = expression {
        // Code exécuté si la correspondance réussit
    } else {
        // Optionnel : Code exécuté si la correspondance échoue
    }
    ```

-   **🎯 Quand utiliser `if let` ?**

    -   Quand on veut **extraire une valeur** d'un `Some`, `Ok`, ou d'une variante d'`enum`.
    -   Pour éviter d'écrire un `match` complet quand on ne gère qu'un seul cas.
    -   Pour simplifier le code quand on ignore les autres possibilités.

-   **Exemple : Traiter un `Option` avec `if let`**

    ```rust
    fn main() {
        let nombre: Option<i32> = Some(42);

        // Avec `match` (plus verbeux)
        match nombre {
            Some(x) => println!("Le nombre est {}", x),
            None => (), // On ne fait rien
        }

        // Avec `if let` (plus concis)
        if let Some(x) = nombre {
            println!("Le nombre est {}", x); // Affiche "Le nombre est 42"
        }
    }
    ```

-   **Exemple : Utilisation avec des `enum`**

    ```rust
    enum Message {
        Quitter,
        Ecrire(String),
        Deplacer { x: i32, y: i32 },
    }

    fn main() {
        let msg = Message::Ecrire(String::from("Salut !"));

        if let Message::Ecrire(texte) = msg {
            println!("Message reçu : {}", texte); // Affiche "Message reçu : Salut !"
        }
    }
    ```
