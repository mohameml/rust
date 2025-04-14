# Cour 02 : **Les Variables:**

## 1.**1️⃣ Variables non initialisées**

-   **Définition**

    > Une variable **non initialisée** est déclarée mais sans valeur assignée. En Rust, le compilateur interdit leur utilisation pour éviter des comportements indéfinis.

-   **Syntaxe**

    ```rust
    let x: i32; // Non initialisée (erreur si utilisée)
    let _y: i32; // Non initialisée mais ignorée (avertissement)
    ```

-   **Exemple**

    ```rust
    #[allow(unused_variables)]
    fn main() {
        let x: i32 = 5; // Initialisée ✅
        let _y: i32 = 6; // Ignorée (warning si non utilisée) ⚠️
        assert_eq!(x, 5);
        println!("Success!");
    }
    ```

## 2.**2️⃣ Variables mutables**

-   **Définition**

    > Par défaut, les variables en Rust sont **immutables**. Pour les modifier, utilisez `mut`.

-   **Syntaxe**

    ```rust
    let mut x = 1; // Mutable
    x += 1; // Modification autorisée
    ```

-   **Exemple**

    ```rust
    fn main() {
        let mut z = 1;
        z += 2;
        assert_eq!(z, 3); // z vaut maintenant 3
        println!("Success!");
    }
    ```

## 3.**3️⃣ Portée (Scope)**

-   **Définition**

    > Une variable existe **seulement dans son bloc (`{}`)**. Hors de ce bloc, elle est détruite.

-   **Exemple**

    ```rust
    fn main() {
        let x2 = 10;
        {
            let y2 = 5;
            println!("x: {}, y: {}", x2, y2); // ✅ y2 visible ici
        }
        // println!("y: {}", y2); // ❌ y2 n'existe plus
        println!("x: {}", x2); // ✅ x2 toujours visible
    }

    fn define_x() {
        let x3 = "hello";
        println!("{}, world", x3); // x3 visible uniquement ici
    }
    ```

## 4.**4️⃣ Shadowing (Réutilisation de noms)**

-   **Définition**

    > Rust permet de **redéfinir** une variable avec le même nom, masquant la précédente.

-   **Syntaxe**

    ```rust
    let x = 5;
    let x = x + 1; // Nouveau x (shadowing)
    ```

-   **Exemple**

    ```rust
    fn main() {
        let x4 = 5;
        {
            let x4 = 12; // Shadowing dans ce bloc
            assert_eq!(x4, 12);
        }
        assert_eq!(x4, 5); // Le x4 original est intact

        let x4 = 42; // Nouveau shadowing
        println!("{}", x4); // Affiche 42
    }
    ```

## 5.**5️⃣ Variables inutilisées**

-   **Définition**

    > Rust émet un **avertissement** pour les variables déclarées mais non utilisées. Utilisez `_` pour les ignorer.

-   **Syntaxe**

    ```rust
    let _x = 5; // Pas d'avertissement
    // ou :
    #[allow(unused_variables)]
    ```

-   **Exemple**

    ```rust
    #[allow(unused_variables)]
    fn main() {
        let _x7 = 5; // Ignorée volontairement
    }
    ```

## 6.**6️⃣ Destructuration (Destructuring)**

-   **Définition**

    > Permet de **décomposer** un tuple ou un tableau en variables individuelles.

-   **Syntaxe**

    ```rust
    let (x, y) = (1, 2); // x=1, y=2
    ```

-   **Exemple**

    ```rust
    fn main() {
        let (mut x, y) = (1, 2);
        x += 2;
        assert_eq!(x, 3);
        assert_eq!(y, 2);
        println!("Success!");
    }
    ```

## 7.**7️⃣ Destructuration avancée**

-   **Définition**

    > Utilisation de `..` pour ignorer des éléments lors de la destructuration.

-   **Syntaxe**

    ```rust
    let (x, ..) = (3, 4); // x=3, ignore 4
    let [.., y] = [1, 2]; // y=2, ignore 1
    ```

-   **Exemple**

    ```rust
    fn main() {
        let (x, y);
        (x, ..) = (3, 4); // x=3
        [.., y] = [1, 2]; // y=2
        assert_eq!([x, y], [3, 2]);
        println!("Success!");
    }
    ```
