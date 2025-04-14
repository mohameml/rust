# Cour 1 : **Cours d'Introduction à Rust**

## **1. Introduction à Rust**

-   **Qu’est-ce que Rust ?**

    > Rust est un **langage de programmation système** moderne, développé par Mozilla, axé sur **la performance, la sécurité mémoire et le parallélisme**. Il combine :

    -   La vitesse du **C/C++**.
    -   La sûreté mémoire **sans garbage collector**.
    -   Une syntaxe moderne et expressive.

-   **À quoi sert Rust ?**

    -   🖥 **Programmation système** (OS, drivers, outils bas niveau).
    -   🌐 **Serveurs performants** (WebAssembly, backend).
    -   🔧 **Outils CLI et embarqués**.
    -   🎮 **Game Dev** (moteurs comme Bevy).
    -   ⛓ **Blockchain** (Solana, Polkadot).

-   **Avantages / Inconvénients**

    | **Avantages ✅**                                                   | **Inconvénients ❌**                     |
    | ------------------------------------------------------------------ | ---------------------------------------- |
    | ✔ **Sécurité mémoire** (pas de _dangling pointers_, _data races_). | ✖ Courbe d’apprentissage raide.          |
    | ✔ **Performances proches du C**.                                   | ✖ Écosystème moins mature que Python/JS. |
    | ✔ **Gestion explicite des erreurs** (`Result`, `Option`).          | ✖ Compilation parfois lente.             |
    | ✔ **Outillage excellent** (`cargo`, `clippy`, `rustfmt`).          | ✖ Moins adapté aux scripts rapides.      |

## **2. Installation (sur Linux/Unix)**

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

-   Ajoutez Rust au `PATH` :
    ```sh
    source "$HOME/.cargo/env"
    ```
-   Vérifiez :
    ```sh
    rustc --version  # Affiche la version (ex: rustc 1.70.0)
    ```

## **3. Exécution d’un programme Rust**

### **Méthode 1 : Directe (`rustc`)**

```sh
echo 'fn main() { println!("Hello"); }' > main.rs
rustc main.rs  # Génère un binaire `main`
./main        # Affiche "Hello"
```

### **Méthode 2 : Avec Cargo (recommandé)**

```sh
cargo new hello_world
cd hello_world
cargo run     # Compile + exécute
```

## **4. Hello World en Rust**

-   **hello.rs:**

    ```rust
    fn main() {
        println!("Hello, world! 🦀");
    }
    ```

-   **Explications** :
    -   `fn main()` : Point d’entrée du programme.
    -   `println!` : Macro pour afficher du texte.

### RQ 🔗 **Ressources** :

-   [The Book (FR)](https://jimskapt.github.io/rust-book-fr/)
-   [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
-   [Rust by Practice](https://practice.course.rs/why-exercise.html)
