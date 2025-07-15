# **Cours Rust : Generic Types (Types génériques)**

## 1. Définition générale de la **programmation générique**

-   **Définition formelle:**

    -   La **programmation générique** est un paradigme qui permet d’écrire des fonctions, structures ou types **indépendants du type de données manipulé**.

    > Elle repose sur la **paramétrisation de type**, permettant d’utiliser un **type générique `T`** à la place d’un type concret (`i32`, `String`, etc.) dans le code.

-   **Objectifs :**

    -   Réduire la duplication de code
    -   Écrire du code **abstrait**, **réutilisable** et **type-safe**
    -   Maintenir la **performance** (résolu à la compilation en Rust, donc zéro coût)

-   Utilité en **real world** :

    -   **Bibliothèques mathématiques** (vecteurs, matrices génériques)
    -   **Systèmes de fichiers** (ex : lecteur de fichiers JSON, CSV, XML)
    -   **Structures de données** (pile, file, graphe, arbre, etc.)
    -   **Mécanismes de parsing** (où l’entrée peut être de n’importe quel type)

## 2. Generics en **fonction**

-   **Syntaxe :**

    ```rust
    fn nom_fonction<T>(param: T) -> T {
        // ...
    }
    ```

    -   `T` est un **paramètre de type générique**
    -   Peut être contraint par des **traits** (ex : `T: Copy`)

-   **Exemple concret : fonction de duplication générique**

    ```rust
    fn dupliquer<T: Clone>(val: T) -> (T, T) {
        (val.clone(), val)
    }

    fn main() {
        let pair = dupliquer("Rust".to_string());
        println!("{:?}", pair); // ("Rust", "Rust")
    }
    ```

## 3. 🏗️ Generics en **struct**

-   **Définition:**

    > Une `struct` peut être paramétrée par un ou plusieurs types génériques.

-   **Syntaxe de base:**

    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    // Implémentation générique
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    ```

-   **Exemple complexe : Cache générique avec fonction de calcul**

    ```rust
    use std::collections::HashMap;

    struct Cache<K, V, F>
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
        F: Fn(K) -> V,
    {
        calcul: F,
        memo: HashMap<K, V>,
    }

    impl<K, V, F> Cache<K, V, F>
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
        F: Fn(K) -> V,
    {
        fn new(calcul: F) -> Self {
            Cache {
                calcul,
                memo: HashMap::new(),
            }
        }

        fn valeur(&mut self, k: K) -> &V {
            self.memo.entry(k.clone()).or_insert_with(|| (self.calcul)(k))
        }
    }

    fn main() {
        let mut cache = Cache::new(|x: u32| {
            println!("Calcul pour {}", x);
            x * 2
        });

        println!("{}", cache.valeur(10)); // Calcul
        println!("{}", cache.valeur(10)); // Pas de recalcul
    }
    ```

## 4. Generics en **enum**

-   **Syntaxe:**

    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```

    -   Très utilisé dans la stdlib (`Option<T>`, `Result<T, E>`, etc.)

-   **Exemple concret : Enum générique pour état de chargement**

    ```rust
    enum Chargement<T, E> {
        EnCours,
        Réussi(T),
        Échoué(E),
    }

    fn charger_données() -> Chargement<String, String> {
        // Imaginons qu'on tente de charger un fichier
        let ok = true;

        if ok {
            Chargement::Réussi("Contenu chargé".to_string())
        } else {
            Chargement::Échoué("Erreur d'I/O".to_string())
        }
    }

    fn main() {
        match charger_données() {
            Chargement::EnCours => println!("Chargement..."),
            Chargement::Réussi(données) => println!("Succès : {}", données),
            Chargement::Échoué(err) => println!("Erreur : {}", err),
        }
    }
    ```

## 5. **Contraintes sur les types génériques** (Trait Bounds)

-   **Définition formelle:**

    > Une **contrainte générique** (ou **trait bound**) restreint le type générique `T` pour qu’il respecte certaines **capacités** définies par un ou plusieurs **traits**.

-   Cela permet :

    -   D'utiliser des méthodes ou comportements spécifiques (comme `.clone()`, `.to_string()`, etc.)
    -   De s'assurer à la compilation que les types utilisés **respectent certaines interfaces**

-   **Syntaxe abrégée**

    ```rust
    fn fonction<T: Trait>(arg: T) { ... }
    ```

-   **Syntaxe `where`** (meilleure lisibilité si plusieurs contraintes)

    ```rust
    fn fonction<T, U>(arg1: T, arg2: U)
    where
        T: Trait1 + Trait2,
        U: Trait3,
    {
        // ...
    }
    ```

-   **Traits standard les plus courants :**

    | Trait               | Description courte                                |
    | ------------------- | ------------------------------------------------- |
    | `Clone`             | Permet de dupliquer un objet (`.clone()`)         |
    | `Copy`              | Permet la copie binaire implicite (types simples) |
    | `Debug`             | Permet d’imprimer avec `{:?}`                     |
    | `PartialEq`         | Permet la comparaison avec `==`, `!=`             |
    | `Ord`, `PartialOrd` | Permet de trier ou comparer                       |
    | `Default`           | Permet de générer une valeur par défaut           |
    | `Display`           | Affichage formaté via `{}`                        |
    | `From`, `Into`      | Conversions de types                              |
    | `Hash`              | Pour être utilisé dans `HashMap`, `HashSet`       |

-   **Exemples:**

    ```rust
    fn comparer_et_afficher<T, U>(a: T, b: U)
    where
        T: std::fmt::Display + PartialEq,
        U: std::fmt::Debug,
    {
        if a == a {
            println!("T est égal à lui-même : {}", a);
        }
        println!("Valeur de U : {:?}", b);
    }
    ```

    ```rust
    fn trier_et_afficher<T: Ord + std::fmt::Debug>(mut liste: Vec<T>) {
        liste.sort();
        println!("Liste triée : {:?}", liste);
    }

    fn main() {
        let mut nombres = vec![3, 1, 4, 1, 5];
        trier_et_afficher(nombres);

        let mut noms = vec!["Zoe", "Alice", "Bob"];
        trier_et_afficher(noms);
    }
    ```

    > Fonction générique capable de trier et d’afficher n’importe quel vecteur triable.
