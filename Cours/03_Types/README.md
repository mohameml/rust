# Cour :

## 1. **Integer:**

-   **Définition**

    > Les entiers en Rust sont des types numériques **non fractionnaires** avec plusieurs variantes selon:

    -   La **taille** en bits (8, 16, 32, 64, 128 bits ou architecturelle)
    -   Le **signe** (signé `i` ou non-signé `u`)

-   **Types Standards**

    | Type    | Description                      | Valeur Min                           | Valeur Max                 |
    | ------- | -------------------------------- | ------------------------------------ | -------------------------- |
    | `i8`    | Entier signé 8-bit               | -128                                 | 127                        |
    | `u8`    | Entier non-signé 8-bit           | 0                                    | 255                        |
    | `i16`   | Entier signé 16-bit              | -32,768                              | 32,767                     |
    | `u16`   | Entier non-signé 16-bit          | 0                                    | 65,535                     |
    | `i32`   | Entier signé 32-bit              | -2,147,483,648                       | 2,147,483,647              |
    | `u32`   | Entier non-signé 32-bit          | 0                                    | 4,294,967,295              |
    | `i64`   | Entier signé 64-bit              | -9,223,372,036,854,775,808           | 9,223,372,036,854,775,807  |
    | `u64`   | Entier non-signé 64-bit          | 0                                    | 18,446,744,073,709,551,615 |
    | `i128`  | Entier signé 128-bit             | -2¹²⁷                                | 2¹²⁷-1                     |
    | `u128`  | Entier non-signé 128-bit         | 0                                    | 2¹²⁸-1                     |
    | `isize` | Entier signé archi-dépendant     | Dépend de l'architecture (32/64-bit) |
    | `usize` | Entier non-signé archi-dépendant | Dépend de l'architecture (32/64-bit) |

-   **Syntaxe de Déclaration**

    ```rust
    let x: i32 = 42; // Annotation explicite
    let y = 42i64;   // Suffixe de type
    let z = 42;      // Inférence (i32 par défaut)
    let a = u8::MAX ;
    let b = u8::MIN;
    ```

-   **Exemples**

    ```rust
    fn main() {
        let a: u8 = 255;
        let b: i16 = -32000;
        let c = 100_000; // _ comme séparateur de lisibilité (i32)

        println!("a = {}, b = {}, c = {}", a, b, c);
    }
    ```

## 2. **Floats:**

-   **Définition**

    > Les flottants en Rust sont des types numériques **fractionnaires** qui représentent des nombres réels. Rust propose deux types principaux:

    -   `f32`: 32 bits (précision simple)
    -   `f64`: 64 bits (précision double)

-   **Types Standards**

    | Type  | Description     | Taille   | Précision (~décimaux)        | Plage approximative      |
    | ----- | --------------- | -------- | ---------------------------- | ------------------------ |
    | `f32` | Flottant 32-bit | 4 octets | 6-9 chiffres significatifs   | ±1.2×10⁻³⁸ à ±3.4×10³⁸   |
    | `f64` | Flottant 64-bit | 8 octets | 15-17 chiffres significatifs | ±2.2×10⁻³⁰⁸ à ±1.8×10³⁰⁸ |

-   **Valeurs Spéciales**

    ```rust
    let inf = f32::INFINITY;    // Infini positif
    let neg_inf = f32::NEG_INFINITY; // Infini négatif
    let nan = f32::NAN;         // "Not a Number"
    ```

-   **Syntaxe de Déclaration**

    ```rust
    let x: f32 = 3.14; // Annotation explicite
    let y = 3.14f64;   // Suffixe de type
    let z = 3.14;      // Inférence (f64 par défaut)
    ```

-   **Exemples**

    ```rust
    fn main() {
        let pi: f32 = 3.14159;
        let e = 2.71828f64; // f64 explicite
        let golden_ratio = 1.61803; // f64 par défaut

        println!("π ≈ {}, e ≈ {}, φ ≈ {}", pi, e, golden_ratio);
    }
    ```

    ```rust
    let sum = 5.0 + 10.5;      // Addition
    let diff = 95.5 - 4.3;     // Soustraction
    let product = 4.0 * 30.5;  // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let remainder = 43.0 % 5.0; // Modulo (reste de la division)
    ```

-   **Bonnes Pratiques**

    -   **Précision**: Préférez `f64` sauf pour des cas spécifiques (économiser de la mémoire)

    -   **Comparaisons**: Évitez `==` avec les flottants (utilisez des seuils de tolérance)

        ```rust
        let a = 0.1 + 0.2;
        let b = 0.3;
        assert!((a - b).abs() < 1e-10); // Bonne pratique
        ```

    -   **Performances**: Sur les architectures modernes, `f64` est souvent aussi rapide que `f32`

Voici un **cours structuré sur les types de données (Data Types) en Rust** : `integer`, `float`, `bool`, et `char`, avec définitions, syntaxes, méthodes (dans un tableau), et exemples pratiques.

## 1. **Integer Types**

-   **Définition :**

    > Les types entiers représentent des **valeurs entières** (positives ou négatives, avec ou sans signe).

-   **Syntaxe :**

    ```rust
    let a: i32 = 10;
    let b: u8 = 255;
    ```

-   **Types disponibles :**

    | Type    | Taille  | Signé | Intervalle                        |
    | ------- | ------- | ----- | --------------------------------- |
    | `i8`    | 8 bits  | ✅    | -128 à 127                        |
    | `u8`    | 8 bits  | ❌    | 0 à 255                           |
    | `i16`   | 16 bits | ✅    | -32_768 à 32_767                  |
    | `u16`   | 16 bits | ❌    | 0 à 65_535                        |
    | `i32`   | 32 bits | ✅    | -2^31 à 2^31 - 1                  |
    | `u32`   | 32 bits | ❌    | 0 à 2^32 - 1                      |
    | `i64`   | 64 bits | ✅    | ...                               |
    | `u64`   | 64 bits | ❌    | ...                               |
    | `isize` | machine | ✅    | selon l'architecture (32/64 bits) |
    | `usize` | machine | ❌    | idem                              |

-   **Exemple :**

    ```rust
    fn main() {
        let age: u8 = 25;
        let year: i32 = 2025;
        println!("Age: {}, Year: {}", age, year);
    }
    ```

## 2. **Floating-Point Types (float)**

-   **Définition :**

    > Les nombres à **virgule flottante** permettent de représenter des réels.

-   **Syntaxe :**

    ```rust
    let x: f32 = 3.14;
    let y: f64 = 2.718281828;
    ```

-   **Types disponibles :**

    | Type  | Taille  | Précision                            |
    | ----- | ------- | ------------------------------------ |
    | `f32` | 32 bits | \~6-7 chiffres                       |
    | `f64` | 64 bits | \~15-16 chiffres (valeur par défaut) |

-   **Méthodes utiles :**

    | Méthode    | Description                  | Exemple               |
    | ---------- | ---------------------------- | --------------------- |
    | `.abs()`   | Valeur absolue               | `(-3.5f64).abs()`     |
    | `.sqrt()`  | Racine carrée                | `9.0f64.sqrt()`       |
    | `.powf(x)` | Puissance réelle             | `2.0f64.powf(3.0)`    |
    | `.floor()` | Plancher (arrondi inférieur) | `3.8.floor()` → `3.0` |
    | `.ceil()`  | Plafond (arrondi supérieur)  | `3.2.ceil()` → `4.0`  |
    | `.round()` | Arrondi                      | `3.5.round()` → `4.0` |

-   **Exemple :**

    ```rust
    fn main() {
        let pi: f64 = 3.14159;
        let radius = 5.0;
        let area = pi * radius.powf(2.0);
        println!("Aire = {}", area);
    }
    ```

## 3. **Boolean (bool)**

-   **Définition :**

    > Représente une valeur **vraie ou fausse**.

-   **Syntaxe :**

    ```rust
    let is_active: bool = true;
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let is_even = 4 % 2 == 0;
        if is_even {
            println!("Nombre pair !");
        }
    }
    ```

## 4. **Character (char)**

-   **Définition :**

    > Un caractère Unicode, entre **' '** (apostrophes), sur 4 octets. Peut contenir des emojis et symboles.

-   **Syntaxe :**

    ```rust
    let lettre: char = 'A';
    let emoji: char = '😎';
    ```

-   **Méthodes utiles :**

    | Méthode            | Description                      | Exemple               |
    | ------------------ | -------------------------------- | --------------------- |
    | `.is_alphabetic()` | Vérifie si c’est une lettre      | `'a'.is_alphabetic()` |
    | `.is_numeric()`    | Vérifie si c’est un chiffre      | `'7'.is_numeric()`    |
    | `.to_uppercase()`  | Majuscule (renvoie un itérateur) | `'a'.to_uppercase()`  |
    | `.len_utf8()`      | Taille UTF-8 en octets           | `'€'.len_utf8()` → 3  |

-   **Exemple :**

    ```rust
    fn main() {
        let heart: char = '❤';
        println!("Char: {}", heart);
    }
    ```
