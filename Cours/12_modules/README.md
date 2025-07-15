# Cour : **Modules**

## 1 **Modules:**

-   **Def:**

    > En Rust, un module est une unité d’organisation du code à l’intérieur d’une crate.Il sert à organiser, structurer et cacher ou exposer certaines parties du code (fonctions, structs, enums, etc.).

-   **Objectifs d’un module :**

    -   Diviser le code en plusieurs fichiers ou sections logiques.
    -   Rendre certaines parties privées ou publiques (`pub`).
    -   Réduire la complexité des gros fichiers.

-   **Syntaxe de base:**

    -   **Déclaration d’un module **interne** (dans le même fichier) :**

        ```rust
        mod util {
            pub fn saluer() {
                println!("Bonjour !");
            }
        }

        fn main() {
            util::saluer(); // Appel à la fonction dans le module
        }
        ```

    -   **Déclaration d’un module **externe** (dans un autre fichier) :**

        ```rust
        // main.rs
        mod util;

        fn main() {
            util::saluer();
        }
        ```

        ```rust
        // util.rs (dans src/)
        pub fn saluer() {
            println!("Bonjour !");
        }
        ```

    -   **Modules imbriqués (hiérarchiques):**

        ```
        src/
        ├── main.rs
        ├── reseau/
        │   ├── mod.rs
        │   └── client.rs

        <!-- ou :  -->
        src/
        ├── main.rs
        ├── reseau.rs
        ├── reseau/
        │   └── client.rs

        ```

        ```rust
        // main.rs
        mod reseau;

        fn main() {
            reseau::client::connecter();
        }
        ```

        ```rust
        // reseau/mod.rs
        // or in : src/reseau.rs
        pub mod client;
        ```

        ```rust
        // reseau/client.rs
        pub fn connecter() {
            println!("Client connecté !");
        }
        ```

-   **Visibilité: `pub`**

    -   Tout ce qui est déclaré **dans un module** est **privé par défaut**.
    -   Il faut utiliser `pub` pour exposer des éléments à l’extérieur du module.

    ```rust
    mod math {
        fn privee() {}       // Non accessible en dehors
        pub fn publique() {} // Accessible
    }
    ```

## 2. **Crate:**

-   **Définition:**

    > En Rust, un **crate** est une unité de compilation et de distribution. C'est le plus petit élément que le compilateur Rust traite individuellement.

-   **Types de crate:**

    -   **Une bibliothèque (library crate)** :

        -   crée avec `cargo new --lib mon_crate`
        -   Fournit des fonctionnalités réutilisables .
        -   Généralement compilé en un fichier `.rlib`.
        -   Exemple : `serde` (pour la sérialisation), `rand` (pour l'aléatoire).

    -   **Un exécutable (binary crate)** :
        -   Contient une fonction `main()` et produit un programme exécutable.
        -   Par exemple, un projet créé avec `cargo new --bin mon_projet`.

-   **Caractéristiques d'un crate :**

    -   **Empaquetage** : Un crate peut contenir plusieurs modules, mais il a un point d'entrée (`lib.rs` pour une bibliothèque, `main.rs` pour un binaire).

    -   **Gestion des dépendances** : Dans `Cargo.toml`, on déclare les crates externes utilisés.

    -   **Publication** : Les crates peuvent être publiés sur [crates.io](https://crates.io) (le registre officiel de Rust).

## 3. **Package :**

-   **Définition:**

    -   Un **package** est un _ensemble de crates_ (au moins une) qui sont gérées ensemble avec un `Cargo.toml`.

    -   Un package **doit contenir au moins une crate**, et **peut contenir** :

        -   Une **crate de bibliothèque** (`lib.rs`)
        -   Plusieurs **crates binaires** (`main.rs` ou fichiers dans `src/bin/`)

    > **Commande** : `cargo new nom_package` → crée un package avec une crate binaire par défaut.

-   Un package contient :

    -   Un fichier **`Cargo.toml`** (obligatoire) : définit le nom, la version, les dépendances, etc.
    -   Un dossier **`src/`** qui contient les **fichiers source** des crates.
    -   Par défaut :

        -   `src/main.rs` → définit une **crate binaire**
        -   `src/lib.rs` → définit une **crate bibliothèque**

-   Règles importantes d’un **package** :

    -   Un package peut **contenir au maximum une crate bibliothèque** (`lib.rs`).
    -   Il peut contenir **autant de crates binaires que nécessaire** (dans `src/bin/*.rs`).
    -   Un package doit contenir **au moins une crate** (binaire ou lib).

-   **Compilation :**

    -   `Compiler le binaire principal :`

        ```
        cargo run
        ```

    -   `Compiler un binaire spécifique :`

        ```
        cargo run --bin outil1
        ```

    -   `Compiler la bibliothèque uniquement :`

        ```
        cargo build --lib
        ```

-   **Exemple simple : 1 package = 1 crate binaire**

    ```
    mon_app/
    ├── Cargo.toml     ← le package
    └── src/
        └── main.rs    ← la crate binaire
    ```

-   **Exemple : 1 package = 1 lib + plusieurs binaires**

    ```
    mon_package/
    ├── Cargo.toml     ← le package
    └── src/
        ├── lib.rs     ← crate bibliothèque
        ├── main.rs    ← crate binaire principale (par défaut)
        └── bin/
            ├── outil1.rs   ← autre binaire
            └── outil2.rs   ← autre binaire
    ```

## 4. **Importer les modules en Rust:**

-   **Chemin absolu** (`crate::...`)

    > Commence toujours par le **point d’entrée** du projet (`crate` = racine de ton crate).

    ```rust
    crate::reseau::client::connecter();
    ```

-   **Chemin relatif**

    > Commence depuis le **module courant** (comme les chemins relatifs de fichiers).

    -   `self::` → module actuel
    -   `super::` → module parent
    -   `nom_mod::` → sous-module du courant

    Exemple depuis `reseau/mod.rs` :

    ```rust
    self::client::connecter();     // ou simplement client::connecter();
    super::utils::log();          // module frère de `reseau`
    ```

-   **Le mot-clé `use`:**

    > Le mot-clé `use` permet d’**importer une fonction, module ou struct** pour éviter d’écrire tout le chemin à chaque fois.

    ```rust
    use crate::reseau::client;
    use std::io::{self , Write};
    use std::io::*;

    fn main() {
        client::connecter();
    }
    ```

-   **Import multiple**

    ```rust
    use crate::reseau::client::{connecter, deconnecter};
    ```

-   **Renommer une importation**

    ```rust
    use crate::reseau::client::connecter as connexion;

    fn main() {
        connexion();
    }
    ```

## 5. **Les règles de **visibilité (privacy)** en Rust:**

-   **Principe de base :**

    > 🔐 **Tout est privé par défaut en Rust.**

    -   Un **élément dans un module** (fonction, struct, constante, etc.) est **privé** à ce module, sauf si on l’exporte avec `pub`.
    -   Seuls les **modules enfants ont accès à leur parent**, **pas l'inverse**.

-   **Le mot-clé `pub`**

    -   `pub` = rend un élément accessible **au module parent ou au monde extérieur**
    -   Sans `pub`, l'élément est **invisible depuis l’extérieur du module**

    ```rust
    mod math {
        fn cachee() {}           // pas visible ailleurs
        pub fn publique() {}     // visible depuis le parent
    }

    fn main() {
        // math::cachee();     ❌ Erreur : cachee est privée
        math::publique();        // ✅ OK
    }
    ```

-   **Accès entre modules (parent ↔ enfant)**

    -   ✅ **Un module enfant** peut accéder aux fonctions **privées de son parent**
    -   ❌ **Un parent** ne peut **pas accéder** aux éléments d’un enfant **non `pub`**

    ```rust
    mod parent {
        fn interne() {
            println!("appelée !");
        }

        pub mod enfant {
            pub fn utiliser_parent() {
                super::interne(); // ✅ OK, le parent est accessible
            }
        }
    }
    ```

    ```rust
    mod parent {
        mod enfant {
            pub fn f() {}
        }

        fn test() {
            enfant::f(); // ❌ Erreur : `enfant` est privé
        }
    }
    ```

-   **Structs et visibility**

    -   Une `struct` publique n'expose **pas ses champs**
    -   Tu dois marquer **chaque champ `pub`** si tu veux les rendre visibles

    ```rust
    pub struct Personne {
        pub nom: String,       // visible
        age: u32,              // privé
    }

    fn main() {
        let p = Personne {
            nom: "Ali".into(),
            age: 30,           // ❌ Erreur : champ privé
        };
    }
    ```

-   **Enums sont spéciaux**

    > Quand une `enum` est `pub`, **toutes ses variantes sont aussi publiques** par défaut.

    ```rust
    pub enum Couleur {
        Rouge,
        Vert,
        Bleu,
    }

    fn main() {
        let c = Couleur::Rouge; // ✅ OK si `Couleur` est pub
    }
    ```

-   **Variantes avancées de `pub`:**

    | Syntaxe                     | Signification                                  |
    | --------------------------- | ---------------------------------------------- |
    | `pub`                       | Visible partout (`crate`, extern, etc.)        |
    | `pub(crate)`                | Visible uniquement dans le **crate** courant   |
    | `pub(super)`                | Visible seulement dans le **module parent**    |
    | `pub(in chemin::vers::mod)` | Visible uniquement dans le **module spécifié** |

    ```rust
    mod a {
        pub(crate) fn interne() {}  // visible dans tout le crate
        pub(super) fn vers_parent() {} // visible uniquement dans le parent
    }
    ```

## 6. **`pub use:`:**

-   **Définition** :

    > `pub use` permet **d’importer** un élément ET de le **réexporter**, pour qu’il soit accessible depuis d’autres modules.

-   **Syntaxe** :

    ```rust
    pub use chemin::vers::élément;
    ```

-   **Exemple concret** :

    ```
    src/
    ├── main.rs
    └── util/
        ├── mod.rs
        └── outil.rs
    ```

    ```rust
    // `util/outil.rs`
    pub fn helper() {
        println!("Outil appelé !");
    }
    ```

    ```rust
    //`util/mod.rs`
    pub mod outil;
    pub use outil::helper; // ✅ Réexporte helper
    ```

    ```rust
    // `main.rs`
    mod util;

    fn main() {
        util::helper(); // ✅ Appel direct grâce à pub use
    }
    ```
