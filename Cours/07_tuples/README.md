# Cour 07 : **Tuples**

## 1. Introduction aux **tuples**

-   **Définition :**

    > Un **tuple** est une **collection de valeurs hétérogènes** (de types différents), groupées ensemble dans une seule structure.

    -   La taille et le type des éléments sont **fixés** à la compilation.
    -   Les tuples sont **stockés sur la stack**.
    -   Ils sont souvent utilisés pour **regrouper des valeurs** sans créer de structure personnalisée.

-   **Syntaxe :**

    ```rust
    let nom: (Type1, Type2, ...) = (valeur1, valeur2, ...);
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let personne: (&str, i32, f32) = ("Alice", 30, 1.65);
        println!("Nom: {}, Âge: {}, Taille: {}", personne.0, personne.1, personne.2);
    }
    ```

## 2. **Accès et déstructuration:**

-   **Accès par index :**

    > Les éléments sont accédés avec une **notation par point** : `.0`, `.1`, `.2`, etc.

    ```rust
    fn main() {
        let coord = (4, 7, 9);
        println!("x: {}, y: {}, z: {}", coord.0, coord.1, coord.2);
    }
    ```

-   **Déstructuration (Pattern Matching) :**

    On peut **extraire les éléments** directement dans des variables.

    ```rust
    let (x, y, z) = coord;
    ```

    ```rust
    fn main() {
        let point = (10, 20);
        let (x, y) = point;
        println!("x: {}, y: {}", x, y);
    }
    ```
