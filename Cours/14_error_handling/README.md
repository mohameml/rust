# **Cours Rust : Error Handling (Gestion des erreurs)**

## 1. **Introduction :**

> Rust distingue **deux grandes familles d’erreurs** :

-   **Unrecoverable Errors** (Erreurs irrécupérables)

    -   Ce sont des erreurs **fatales**, qui font planter le programme.
    -   Elles se produisent dans des situations où il n’est **pas possible de continuer l’exécution** en toute sécurité.

    -   **Exemples :**

        -   Débordement d’indice dans un tableau (`vec[99]` alors qu’il n’a que 5 éléments)
        -   Panic explicite avec `panic!()`

-   **Recoverable Errors** (Erreurs récupérables)

    -   Ce sont des erreurs **attendues**, que le programme peut gérer proprement.
    -   Elles sont modélisées avec l’énumération `Result<T, E>`.

    -   **Exemples :**

        -   Échec d’ouverture de fichier (le fichier n’existe pas)
        -   Connexion réseau échouée

## 2. **`panic!`:**

-   **Définition:**

    -   La macro `panic!()` est utilisée pour **arrêter immédiatement** le programme en cas d’erreur fatale.

    -   Elle provoque un **“unwind” de la pile** (stack unwinding) et imprime un message d’erreur.

-   **`RUST_BACKTRACE=1`:**

    -   Permet d’afficher **le backtrace complet** (trace de pile) pour comprendre où le panic a eu lieu.

-   **Syntaxe:**

    ```rust
    fn main() {
        panic!("Quelque chose de grave s’est produit !");
    }
    ```

-   **Exemple:**

    ```rust
    fn get_user_id(username: &str) -> u32 {
        if username.is_empty() {
            panic!("Le nom d'utilisateur ne peut pas être vide !");
        }
        42 // ID fictif
    }

    fn main() {
        let _id = get_user_id(""); // provoque un panic
    }
    ```

    ```bash
    RUST_BACKTRACE=1 cargo run
    ```

## 3. **`Result<T, E>`:**

-   **Définition:**

    > `Result<T, E>` est une **enum générique** représentant le **résultat d’une opération** qui peut échouer.

-   **Syntaxe:**

    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

    -   `Ok(T)` : le résultat est correct
    -   `Err(E)` : une erreur est survenue

-   **Syntaxe d’utilisation:**

    ```rust
    fn lire_fichier(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    ```

-   **Exemple :**

    ```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn lire_contenu_fichier(nom: &str) -> Result<String, io::Error> {
        let mut fichier = File::open(nom)?;
        let mut contenu = String::new();
        fichier.read_to_string(&mut contenu)?;
        Ok(contenu)
    }

    fn main() {
        match lire_contenu_fichier("config.txt") {
            Ok(data) => println!("Fichier : {}", data),
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }
    ```

## 4. **Méthodes de `Result`:**

| Méthode              | Description                                         | Exemple                                |
| -------------------- | --------------------------------------------------- | -------------------------------------- |
| `is_ok()`            | Retourne `true` si le résultat est `Ok(_)`          | `res.is_ok()`                          |
| `is_err()`           | Retourne `true` si le résultat est `Err(_)`         | `res.is_err()`                         |
| `unwrap()`           | Récupère la valeur `Ok`, panic si `Err`             | `res.unwrap()`                         |
| `expect(msg)`        | Comme `unwrap()` mais avec message personnalisé     | `res.expect("Erreur de lecture")`      |
| `unwrap_or(default)` | Récupère `Ok`, sinon retourne une valeur par défaut | `res.unwrap_or("valeur")`              |
| `map(f)`             | Applique une fonction si `Ok`, laisse `Err`         | `res.map( \| x \| x + 1)`              |
| `and_then(f)`        | Enchaîne un second `Result`                         | `res.and_then( \| x \| autre_func(x))` |
| `ok()`               | Convertit `Result<T, E>` en `Option<T>`             | `res.ok()`                             |
| `err()`              | Convertit `Result<T, E>` en `Option<E>`             | `res.err()`                            |

## 5. **Error Propagation (Propagation des erreurs)**

-   **Définition:**

    -   La propagation d’erreur permet de **remonter une erreur à l’appelant** sans la gérer immédiatement.

    -   Plutôt que de faire un `match`, on utilise l'opérateur `?` pour **retourner directement l’erreur**.

-   **l'opérateur `?`:**

    -   l'opérateur `?` est un opérateur de propagation d'erreurs (error propagation operator). Il est utilisé pour simplifier la gestion des erreurs dans les fonctions qui retournent des types Result ou Option.

    -   Fonctionnement avec **Result<T, E>:**

        -   Déroule (`unwrap`) la valeur si le Result est `Ok(T)`

        -   Retourne immédiatement l'erreur `Err(E)` si le Result est `Err(E)`

-   **Syntaxe avec `?`:**

    ```rust
    fn lire_fichier(path: &str) -> Result<String, std::io::Error> {
        let contenu = std::fs::read_to_string(path)?; // retourne l'erreur si elle existe
        Ok(contenu)
    }
    ```

-   **Exemple :**

    ```rust
    use std::fs::File;
    use std::io::{self, BufRead};

    fn lire_premiere_ligne(path: &str) -> Result<String, io::Error> {
        let fichier = File::open(path)?;
        let mut lignes = io::BufReader::new(fichier).lines();
        let premiere = lignes.next().unwrap_or(Ok(String::from("Fichier vide")))?;
        Ok(premiere)
    }

    fn main() {
        match lire_premiere_ligne("data.txt") {
            Ok(ligne) => println!("Première ligne : {}", ligne),
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }
    ```
