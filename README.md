# rust

> une **roadmap** pour apprendre Rust, structurée en plusieurs étapes progressives. Cette roadmap est conçue pour les débutants comme pour ceux qui ont déjà de l'expérience en programmation.

## **📌 Étape 0 : Prérequis**

-   Avoir des bases en programmation (variables, boucles, fonctions, etc.).
-   Comprendre les concepts de base de la mémoire (pile vs tas, ownership, etc.).
-   Installer Rust :
    -   Sur Linux/macOS : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    -   Sur Windows : Télécharger [rustup-init.exe](https://rustup.rs/)

---

## **🚀 Étape 1 : Découverte de Rust (1-2 semaines)**

### **1. Syntaxe de base**

-   Variables (`let`, `mut`, `const`)
-   Types de données (`i32`, `f64`, `bool`, `char`, `&str`, `String`)
-   Structures de contrôle (`if`, `match`, `loop`, `while`, `for`)
-   Fonctions et modules (`fn`, `mod`, `pub`)

### **2. Gestion de la mémoire**

-   Ownership, Borrowing, Lifetimes (`&`, `'a`)
-   Références (`&T`, `&mut T`)
-   La règle des **"un seul mutable ou plusieurs immutables"**

### **3. Structures et Enums**

-   `struct`, `impl`, `trait`
-   `enum` et `Option`, `Result`

### **📚 Ressources recommandées :**

-   [The Rust Book (en)](https://doc.rust-lang.org/book/) (ou [version française](https://jimskapt.github.io/rust-book-fr/))
-   [Rustlings](https://github.com/rust-lang/rustlings) (exercices pratiques)
-   [Exercism Rust Track](https://exercism.org/tracks/rust)
-   [practice_course](https://practice.course.rs/why-exercise.html)

---

## **🔧 Étape 2 : Approfondissement (2-4 semaines)**

### **1. Gestion des erreurs**

-   `unwrap()`, `expect()`, `?`
-   `Result<T, E>` et gestion propre des erreurs

### **2. Traits et génériques**

-   `impl Trait`, `dyn Trait`
-   Génériques (`<T>`)
-   Traits courants (`Clone`, `Debug`, `Default`, `Iterator`)

### **3. Gestion des collections**

-   `Vec`, `HashMap`, `HashSet`
-   Itérateurs (`iter()`, `into_iter()`, `iter_mut()`)

### **4. Tests et documentation**

-   Tests unitaires (`#[test]`, `assert!`)
-   Documentation (`///`, `//!`, `cargo doc --open`)

### **📚 Ressources recommandées :**

-   [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
-   [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

---

## **⚙️ Étape 3 : Programmation système et concurrence (3-5 semaines)**

### **1. Gestion de la concurrence**

-   Threads (`std::thread`)
-   `Arc`, `Mutex`, `RwLock`
-   `async`/`await` avec Tokio ou async-std

### **2. Manipulation avancée de la mémoire**

-   Pointeurs intelligents (`Box`, `Rc`, `Arc`)
-   Unsafe Rust (`unsafe`, pointeurs nus)

### **3. Interaction avec le système**

-   Fichiers (`std::fs`)
-   Réseau (`std::net`, `TcpStream`)

### **📚 Ressources recommandées :**

-   [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (pour `unsafe` Rust)
-   [Tokio Documentation](https://tokio.rs/) (pour l'asynchrone)

---

## **🛠 Étape 4 : Projets pratiques**

### **Mini-projets pour s'entraîner :**

1. Un analyseur de fichiers JSON/TOML.
2. Un serveur HTTP simple avec `hyper` ou `actix-web`.
3. Un jeu en terminal avec `crossterm` ou `ratatui`.
4. Un outil CLI avec `clap`.

### **Projets open-source :**

-   Contribuer à des projets Rust sur GitHub (ex: `ripgrep`, `alacritty`).
-   Participer à [This Week in Rust](https://this-week-in-rust.org/).

---

## **🚀 Étape 5 : Spécialisation (Optionnel)**

Selon vos intérêts :

-   **Embedded** : [Embedded Rust Book](https://docs.rust-embedded.org/book/)
-   **WebAssembly** : [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/)
-   **Blockchain** : [Solana](https://solana.com/) (en Rust)
-   **Game Dev** : [Bevy Engine](https://bevyengine.org/)

---

## **🔗 Liens utiles**

-   [Rust Documentation](https://doc.rust-lang.org/std/)
-   [Rust Playground](https://play.rust-lang.org/)
-   [r/rust (Reddit)](https://www.reddit.com/r/rust/)
-   [Rust Community Discord](https://discord.gg/rust-lang)

---

### **📌 Conseils :**

✅ **Pratiquez régulièrement** (Rust a une courbe d'apprentissage raide).  
✅ **Lisez le code des autres** (GitHub, crates.io).  
✅ **Ne paniquez pas face aux erreurs du compilateur** : elles sont très explicites !

Bonne chance dans votre apprentissage de Rust ! 🦀🚀
