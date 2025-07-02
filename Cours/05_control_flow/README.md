# Cour : **les conditions et boucles en Rust**

## 1. **`if`, `else if`, `else`**

-   **Définition :**

    > Permet de **prendre des décisions** dans le code selon des conditions booléennes.

-   **Syntaxe :**

    ```rust
    if condition {
        // bloc exécuté si condition est vraie
    } else if autre_condition {
        // bloc exécuté si autre_condition est vraie
    } else {
        // bloc exécuté si aucune condition précédente n'est vraie
    }
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let x = 10;

        if x < 5 {
            println!("x est inférieur à 5");
        } else if x == 10 {
            println!("x est égal à 10");
        } else {
            println!("x est autre chose");
        }
    }
    ```

## 2. **`loop`, `break`, `continue`:**

-   **Définition :**

    -   `loop` : crée une boucle **infinie** jusqu'à un `break`.
    -   `break` : **interrompt** la boucle.
    -   `continue` : **passe à l’itération suivante** sans exécuter la suite du bloc.

-   **Syntaxe :**

    ```rust
    loop {
        if condition {
            break;
        }

        if autre_condition {
            continue;
        }
    }
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let mut count = 0;

        loop {
            count += 1;

            if count == 3 {
                continue; // saute l'affichage du 3
            }

            println!("Count: {}", count);

            if count >= 5 {
                break; // sort de la boucle
            }
        }
    }
    ```

## 3. **`for` loop:**

-   **Définition :**

    > Permet de **parcourir une collection** ou une plage de valeurs.

-   **Syntaxe :**

    ```rust
    for variable in start..end {
        // bloc exécuté à chaque itération
    }
    ```

    -   `start..end` : plage exclusive (`end` non inclus)
    -   `start..=end` : plage inclusive

-   **Exemple :**

    ```rust
    fn main() {
        for i in 1..=5 {
            println!("i: {}", i);
        }

        let array = [10, 20, 30];
        for element in array.iter() {
            println!("element: {}", element);
        }
    }
    ```

## 4. \*\*`while`

-   **Définition :**

    > Boucle tant qu’une **condition** est vraie. Utile quand on ne connaît pas à l’avance le nombre d’itérations.

-   **Syntaxe :**

    ```rust
    while condition {
        // bloc exécuté tant que la condition est vraie
    }
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let mut x = 0;

        while x < 3 {
            println!("x: {}", x);
            x += 1;
        }
    }
    ```
