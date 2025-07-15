# Cour : **Closures**

## 1. **Introduction aux _closures_ en Rust:**

-   **Définition:**

    > En Rust, une **closure** (ou **fonction anonyme**, **lambda**) est une **fonction que l’on peut définir à la volée**, souvent **inline**, et qui **peut capturer son environnement** (les variables autour d’elle).

    -   Elles ressemblent aux fonctions, mais ont quelques particularités :

        -   Elles **peuvent capturer des variables de l'environnement** (comme les fonctions anonymes en JavaScript ou les lambdas en Python).
        -   Elles sont souvent **utilisées comme arguments à d'autres fonctions**, notamment dans les itérateurs (`map`, `filter`, etc.).
        -   Elles peuvent **implémenter dynamiquement** un ou plusieurs traits (`Fn`, `FnMut`, `FnOnce`), selon la manière dont elles capturent les variables.

-   **Syntaxe générale:**

    ```rust
    let closure_name = |param1, param2| -> ReturnType {
        // corps de la closure
    };
    ```

    Mais souvent, Rust **infère le type des paramètres et du retour**, donc on peut écrire plus simplement :

    ```rust
    let closure_name = |x, y| x + y;
    ```

-   **Closure vs Fonction:**

    ```rust
    fn add_fn(a: i32, b: i32) -> i32 {
        a + b
    }

    let add_closure = |a, b| a + b;
    ```

    -   Les deux ont le **même effet**, mais :

        -   `add_fn` est une **fonction nommée**.
        -   `add_closure` est une **fonction anonyme (closure)** qui peut **capturer l’environnement** (on le verra ensuite).

-   **Exemple 1:**

    ```rust
    fn main() {
        let add = |a, b| a + b;  // une closure qui additionne deux nombres

        let result = add(2, 3);
        println!("2 + 3 = {}", result);
    }
    ```

    ✔ Ici, `add` est une closure :

    -   Elle prend deux arguments.
    -   Elle renvoie leur somme.
    -   Les types sont **inférés automatiquement** par le compilateur.

-   **Exemple 2:**

    ```rust

    fn apply<F>(f: F, x: u32) -> u32
    where
        F: Fn(u32) -> u32,
    {
        f(x)
    }

    fn main() {
        let f = |x: u32| x * 2;
        let res = apply(f, 3);
        println!("res : {}", res);
    }
    ```

## 2. **Modes de capture : `Fn`, `FnMut`, `FnOnce`:**

-   **Définition:**

    > Lorsqu’une **closure utilise une variable de son environnement**, Rust détermine **automatiquement** le **mode de capture** requis parmi ces trois traits :

    | Trait    | Type de capture                   | Mutabilité     | Consommation             |
    | -------- | --------------------------------- | -------------- | ------------------------ |
    | `Fn`     | **Par emprunt** (`&T`)            | Non mutable    | Pas de consommation      |
    | `FnMut`  | **Par emprunt mutable**(`&mut T`) | Mutable        | Pas de consommation      |
    | `FnOnce` | **Par déplacement** (`T`)         | Peut consommer | Oui, consommée à l’appel |

-   `Fn` – **Emprunt non mutable (lecture seule)**

    > Utilisé quand la closure **lit** des variables de l’environnement **sans les modifier**.

    -   **Syntaxe:**

        ```rust
        fn call_fn<F: Fn()>(f: F) {
            f();
        }
        ```

    -   **Exemple:**

        ```rust
        fn main() {
            let message = "Hello";

            let print_msg = || println!("{}", message);  // emprunt non mutable

            call_fn(print_msg); // Appelle la closure
        }

        fn call_fn<F: Fn()>(f: F) {
            f();
        }
        ```

        -   `message` est empruntée par la closure (lecture seule).
        -   Elle peut être appelée plusieurs fois sans erreur.

-   `FnMut` – **Emprunt mutable**

    > Utilisé quand la closure **modifie** une variable capturée.

    -   **Syntaxe:**

        ```rust
        fn call_fnmut<F: FnMut()>(f: &mut F) {
            f();
        }
        ```

    -   **Exemple:**

        ```rust
        fn main() {
            let mut counter = 0;

            let mut incr = || {
                counter += 1;  // mutation
                println!("Counter: {}", counter);
            };

            call_fnmut(&mut incr);  // on passe un emprunt mutable à la closure
            call_fnmut(&mut incr);
        }

        fn call_fnmut<F: FnMut()>(f: &mut F) {
            f();
        }
        ```

        -   `counter` est empruntée **mutablement**.
        -   La closure peut être appelée plusieurs fois, mais elle doit être **mutable**.

-   `FnOnce` – **Déplacement (consommation)**

    > Utilisé quand la closure **consomme une variable**, c’est-à-dire qu’elle en prend la possession avec `move`.

    -   **Syntaxe:**

        ```rust
        fn call_fnonce<F: FnOnce()>(f: F) {
            f();
        }
        ```

    -   **Exemple:**

        ```rust
        fn main() {
            let s = String::from("hello");

            let consume = move || {
                println!("Consumed: {}", s);  // s est déplacée ici
            };

            call_fnonce(consume);  // une seule utilisation possible
        }

        fn call_fnonce<F: FnOnce()>(f: F) {
            f();
        }
        ```

        -   `s` est capturée **par move** (donc **déplacée** dans la closure).
        -   On ne peut **pas réutiliser** la closure après l’appel.

## 3. **le mot clé `move`:**

-   **Définition:**

    > Le mot-clé `move` est utilisé pour **forcer la capture par valeur** (ownership) dans les **closures** (fermetures), au lieu de la capture par référence.

    -   Par défaut, une closure capture les variables de son environnement :
        -   **Par référence immuable (`&T`)** si elle ne les modifie pas.
        -   **Par référence mutable (`&mut T`)** si elle les modifie.

-   **Exemple sans `move`**

    ```rust
    let x = 10;
    let closure = || println!("{}", x); // Capture `x` par référence (`&x`)
    closure(); // OK, `x` est toujours valide
    ```

-   **Cas d'usage principaux :**

    -   **Closure survivant à son environnement**

        > Si la closure doit vivre plus longtemps que les variables capturées :

        ```rust
        let s = String::from("Hello");
        let closure = move || println!("{}", s); // `s` est déplacée dans la closure
        // println!("{}", s); ❌ Erreur : `s` a été déplacée (`move` oblige l'ownership)
        closure(); // OK, la closure possède `s`
        ```

    -   **Passage à un thread (`std::thread::spawn`)**

        > Les threads requièrent `move` car ils peuvent survivre à la fonction parente :

        ```rust
        let data = vec![1, 2, 3];
        std::thread::spawn(move || {
            println!("Data: {:?}", data); // `data` est déplacée dans le thread
        }).join().unwrap();
        // `data` n'est plus utilisable ici
        ```

    -   **Closures retournées depuis une fonction**

        Si une closure est retournée, elle doit capturer ses variables par `move` :

        ```rust
        fn create_closure() -> impl Fn() {
            let msg = String::from("Hello");
            move || println!("{}", msg) // `msg` est déplacée dans la closure
        }
        ```
