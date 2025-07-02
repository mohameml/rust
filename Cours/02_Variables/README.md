# Cour : **Variables**

## 1. **Variables & Mutabilité:**

-   **Définition :**

    > En Rust, les **variables sont immuables** (non modifiables) par défaut. Pour les rendre mutables (modifiables), il faut utiliser le mot-clé `mut`.

-   **Syntaxe :**

    ```rust
    let nom_variable = valeur;          // immuable
    let mut nom_variable = valeur;      // mutable
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let x = 5;
        // x = 6; // ❌ erreur : x est immuable

        let mut y = 10;
        y = 20; // ✅ ok
        println!("y = {}", y);
    }
    ```

-   **Remarque :**

    > Rust privilégie l'immuabilité pour **la sécurité et la prévisibilité du code**. Rendre une variable mutable doit être une décision explicite.

## 2. **Constantes**

-   **Définition :**

    > Une constante est une **valeur fixe** connue à la compilation, **immuable**, **globale ou locale**, déclarée avec `const`.

-   **Syntaxe :**

    ```rust
    const NOM_CONSTANTE: Type = valeur;
    ```

-   **Exemple :**

    ```rust
    const PI: f64 = 3.14159;

    fn main() {
        println!("La valeur de PI est {}", PI);
    }
    ```

-   **Remarque :**

    -   Une constante **doit avoir un type** explicite.
    -   Elle peut être utilisée **n’importe où** dans le programme.
    -   Ne peut pas être le résultat d’une fonction non `const`.

## 3. **Shadowing:**

-   **Définition :**

    Le **shadowing** consiste à redéclarer une variable avec le même nom, ce qui **remplace l’ancienne** dans une nouvelle portée. Cela permet de changer **le type ou la valeur** sans utiliser `mut`.

-   **Syntaxe :**

    ```rust
    let variable = valeur;
    let variable = nouvelle_valeur;
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("x = {}", x); // x = 12
    }
    ```

    Autre exemple : changer le **type** d'une variable :

    ```rust
    fn main() {
        let spaces = "   ";
        let spaces = spaces.len(); // shadowing : maintenant c'est un usize
        println!("spaces = {}", spaces);
    }
    ```

-   **Quand est-ce utile ?**

    -   Quand on veut **réutiliser le même nom** sans mutabilité.
    -   Quand on veut **changer le type ou la signification** d’une variable dans une nouvelle portée.
    -   Idéal pour une **chaîne de transformations successives** lisible et sûre.
