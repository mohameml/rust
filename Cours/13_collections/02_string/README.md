# Cour : **String**

## 📘 1. **Introduction :**

### 1.1 ASCII (American Standard Code for Information Interchange)

-   **Définition :**

    > L’**ASCII** est un **standard d’encodage de caractères** basé sur un codage **sur 7 bits**, définissant **128 symboles** numérotés de 0 à 127. Chaque caractère (lettre, chiffre ou symbole) est représenté par une valeur entière comprise entre 0 et 127.

-   **Exemples :**

    -   `'A'` → 65
    -   `'a'` → 97
    -   `'0'` → 48
    -   `' '` (espace) → 32

-   **Limitations** :

    Ne prend en charge que l'alphabet anglais, les chiffres arabes, quelques ponctuations et caractères de contrôle. Il ne peut pas représenter les caractères accentués (`é`, `ç`, etc.) ni les alphabets non latins (arabe, chinois, etc.).

### 1.2 **Unicode:**

-   **Définition :**

    > L’**Unicode** est un **standard universel d'encodage de texte**, conçu pour représenter **tous les caractères utilisés dans l’écriture humaine**, ainsi que des symboles techniques et ponctuations.

    -   Chaque caractère est associé à un **code point**, noté `U+XXXX`, où `XXXX` est un nombre en hexadécimal.
    -   Par exemple :

        -   `'A'` → `U+0041`
        -   `'é'` → `U+00E9`
        -   `'中'` → `U+4E2D`

-   **Remarque importante** :
    > Unicode définit un **ensemble abstrait de caractères et de symboles**, mais **ne spécifie pas** _comment_ ces symboles doivent être encodés en mémoire. C’est le rôle des formats comme **UTF-8**, **UTF-16**, etc.

### 1.3 UTF-8 (Unicode Transformation Format – 8 bits)

-   **Définition formelle** :

    > **UTF-8** est un **schéma de codage binaire** des points de code Unicode, basé sur des unités de 8 bits (octets). C’est un **encodage à longueur variable**, utilisant **1 à 4 octets** selon le caractère.

    -   Les 128 premiers caractères Unicode (U+0000 à U+007F, correspondant à ASCII) sont encodés sur 1 octet.
    -   Les caractères au-delà sont encodés sur 2, 3 ou 4 octets selon leur valeur.

-   **Exemples** :

    | Caractère | Unicode | UTF-8 (hex)   | Taille   |
    | --------- | ------- | ------------- | -------- |
    | `'a'`     | U+0061  | `61`          | 1 octet  |
    | `'é'`     | U+00E9  | `C3 A9`       | 2 octets |
    | `'€'`     | U+20AC  | `E2 82 AC`    | 3 octets |
    | `'𐍈'`     | U+10348 | `F0 90 8D 88` | 4 octets |

## 2. **String:**

-   **Définition:**

    > En Rust, une `String` est un **type de données** permettant de **stocker une séquence de caractères Unicode**, encodée en **UTF-8**, et **stockée sur le tas** (_heap_).

-   **Syntaxes de création:**

    -   **`String::new()`:**

        > Crée une `String` vide

        ```rust
        let mut s = String::new(); // s = ""
        s.push_str("Hello");
        ```

    -   **`String::from("...")`:**

        > Construit une `String` à partir d’une **littérale de type `&str`**.

        ```rust
        let s = String::from("Bonjour");
        ```

    -   **`"..." .to_string()`:**

        > Méthode équivalente à `String::from(...)` :

        ```rust
        let s = "Salut".to_string();
        ```

    -   **`format!` macro:**

        Concatène et formate des chaînes avec interpolation.

        ```rust
        let name = "Alice";
        let s = format!("Bonjour, {} !", name);
        ```

-   **Exemples :**

    ```rust
    fn main() {
        // Création simple
        let s1 = String::new();
        let s2 = String::from("Hello");
        let s3 = "World".to_string();

        // Concaténation
        let s4 = s2 + " " + &s3; // s2 est déplacée (ownership)
        println!("{}", s4);

        // Formatage
        let name = "Bob";
        let greeting = format!("Hello, {}!", name);
        println!("{}", greeting);
    }
    ```

## 3. **Méthodes:**

-   **Définition && Syntaxe:**

    | Méthode       | Description courte                               | Syntaxe                           |
    | ------------- | ------------------------------------------------ | --------------------------------- |
    | `new`         | Crée une `String` vide                           | `let s = String::new();`          |
    | `from`        | Crée une `String` à partir d’un `&str`           | `let s = String::from("abc");`    |
    | `to_string`   | Convertit un `&str` en `String`                  | `let s = "abc".to_string();`      |
    | `push`        | Ajoute un seul caractère à la fin                | `s.push('!');`                    |
    | `push_str`    | Ajoute une chaîne (`&str`) à la fin              | `s.push_str(" world");`           |
    | `+`           | Concatène deux chaînes (déplace la gauche)       | `let s = s1 + &s2;`               |
    | `format!`     | Formate sans déplacer                            | `let s = format!("{} {}", a, b);` |
    | `len`         | Donne la longueur en octets                      | `let n = s.len();`                |
    | `is_empty`    | Vérifie si la chaîne est vide                    | `s.is_empty()`                    |
    | `clear`       | Vide complètement la chaîne                      | `s.clear();`                      |
    | `replace`     | Remplace une sous-chaîne par une autre           | `s.replace("a", "b")`             |
    | `contains`    | Vérifie si une sous-chaîne est présente          | `s.contains("abc")`               |
    | `starts_with` | Teste le préfixe                                 | `s.starts_with("abc")`            |
    | `ends_with`   | Teste le suffixe                                 | `s.ends_with("xyz")`              |
    | `trim`        | Supprime les espaces en début et fin             | `s.trim()`                        |
    | `split`       | Coupe selon un séparateur, retourne un itérateur | `s.split(" ")`                    |
    | `chars`       | Itère sur les caractères Unicode                 | `for c in s.chars() { ... }`      |
    | `bytes`       | Itère sur les octets (u8)                        | `for b in s.bytes() { ... }`      |
    | `capacity`    | Capacité mémoire allouée (en octets)             | `s.capacity()`                    |
    | `reserve`     | Réserve de l’espace sans changer le contenu      | `s.reserve(10);`                  |

-   **Exemple:**

    > Nettoyer, valider et formater les données d’un utilisateur (`nom`, `email`, `mot de passe`) avant de les stocker.

    ```rust
    fn main() {
        let mut nom = "  Alice Dupont  ".to_string();
        let mut email = "ALICE.DUPONT@example.COM ".to_string();
        let mut mot_de_passe = String::from("  secret123 ");

        // Nettoyage des espaces inutiles
        nom = nom.trim().to_string();               // -> "Alice Dupont"
        email = email.trim().to_lowercase();        // -> "alice.dupont@example.com"
        mot_de_passe = mot_de_passe.trim().to_string(); // -> "secret123"

        // Vérification des champs
        if email.contains('@') && email.ends_with(".com") {
            println!("Email valide.");
        } else {
            println!("Email invalide.");
        }

        // Génération d’un message formaté
        let message = format!(
            "Bienvenue {}, votre compte a été créé avec l’email : {}.",
            nom, email
        );
        println!("{}", message);

        // Effacement du mot de passe après usage (sécurité)
        mot_de_passe.clear(); // mot_de_passe devient vide
    }
    ```
