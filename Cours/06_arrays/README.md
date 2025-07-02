# Cour : **Arrays**

## 1. **Introduction aux arrays:**

-   **Définition :**

    > Un **array** en Rust est une **collection de valeurs** de **même type**, de **taille fixe** connue à la compilation.

    > ⚠ Contrairement aux _vecteurs (`Vec<T>`) qui sont dynamiques et stockés sur le heap_, **les arrays sont statiques** et **stockés sur la stack**.

-   **Syntaxe :**

    ```rust
    let nom: [Type; taille] = [valeur1, valeur2, ...];
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let notes: [i32; 4] = [12, 15, 18, 20];
        println!("Première note : {}", notes[0]);
    }
    ```

    > Rust vérifie les **débordements d’indice** à l'exécution (panic si `notes[100]`).

## 2. **Accès et Parcours:**

-   **Accès à un élément :**

    -   Via un **indice** (commence à 0).
    -   Syntaxe : `array[indice]`

    ```rust
    let x = mon_array[2];
    ```

-   **Parcours d’un array :**

    Utilise `for` ou `.iter()`.

    ```rust
    fn main() {
        let chiffres = [1, 2, 3, 4, 5];

        // Accès
        println!("Deuxième chiffre : {}", chiffres[1]);

        // Parcours avec for
        for i in 0..chiffres.len() {
            println!("Indice {}: {}", i, chiffres[i]);
        }

        // Parcours avec iter
        for val in chiffres.iter() {
            println!("Valeur : {}", val);
        }
    }
    ```

## 3. **Destruction :**

-   **Définition :**

    > On peut **déstructurer un array** pour extraire ses valeurs.

-   **Syntaxe :**

    ```rust
    let [a, b, c] = array;
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let coord = [4, 7, 9];

        let [x, y, z] = coord;
        println!("x: {}, y: {}, z: {}", x, y, z);
    }
    ```

    > ⚠ La taille doit correspondre exactement.

## 4. **Méthodes utiles des arrays:**

Les arrays implémentent certaines méthodes via le trait `Slice`, via `.iter()` :

| Méthode                | Description                        | Exemple               |
| ---------------------- | ---------------------------------- | --------------------- |
| `.len()`               | Longueur de l'array                | `arr.len()`           |
| `.is_empty()`          | Vérifie si vide                    | `arr.is_empty()`      |
| `.iter()`              | Itérateur immuable                 | `for x in arr.iter()` |
| `.contains(&x)`        | Contient un élément ?              | `arr.contains(&3)`    |
| `.first()` / `.last()` | Premier / dernier élément (option) | `arr.first()`         |

-   **Exemple :**

    ```rust
    fn main() {
        let a = [10, 20, 30];

        println!("Taille: {}", a.len());
        println!("Premier élément: {:?}", a.first());
        println!("Contient 20 ? {}", a.contains(&20));
    }
    ```
