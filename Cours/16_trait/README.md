# Cour : **Trait**

## 1. **Définition des `traits` en Rust:**

-   **Définition:**

    > En **Rust**, un `trait` est **une interface** : Il définit un **comportement commun** (une collection de méthodes) que plusieurs types peuvent **implémenter**.

    👉 Cela permet d’écrire du **code générique** basé sur le comportement plutôt que sur le type exact.

-   **Syntaxe générale:**

    > Définir un trait

    ```rust
    trait NomDuTrait {
        fn nom_de_la_methode(&self) -> TypeReturn;
    }
    ```

    > Implémenter un trait pour un type

    ```rust
    struct MonType;

    impl NomDuTrait for MonType {
        fn nom_de_la_methode(&self) -> TypeReturn {
            // implémentation spécifique
        }
    }
    ```

-   **Exemple :**

    > Objectif : définir un trait `Afficher` que plusieurs types peuvent implémenter.

    ```rust
    // Définition du trait
    trait Afficher {
        fn afficher(&self);
    }

    // Un premier type
    struct Personne {
        nom: String,
        age: u8,
    }

    // Implémentation du trait pour Personne
    impl Afficher for Personne {
        fn afficher(&self) {
            println!("Nom: {}, Âge: {}", self.nom, self.age);
        }
    }

    // Un second type
    struct Livre {
        titre: String,
        auteur: String,
    }

    // Implémentation du trait pour Livre
    impl Afficher for Livre {
        fn afficher(&self) {
            println!("Titre: '{}', Auteur: {}", self.titre, self.auteur);
        }
    }

    // Fonction qui accepte n'importe quel type implémentant Afficher
    fn afficher_element<T: Afficher>(item: T) {
        item.afficher();
    }

    fn main() {
        let p = Personne { nom: "Alice".into(), age: 30 };
        let l = Livre { titre: "Rust pour les nuls".into(), auteur: "Jean Rustacean".into() };

        afficher_element(p);
        afficher_element(l);
    }
    ```

## 2. **trait bounds:**

-   **Définition:**

    > Un **Trait Bound** permet de **restreindre** un type générique à ceux qui **implémentent un ou plusieurs traits**.

-   **Exemple d’idée :**

    > "Cette fonction peut accepter n’importe quel type `T`, **à condition** que `T` implémente le trait `Afficher`."

    Cela garantit que les méthodes définies dans le trait sont **disponibles** sur les types utilisés.

-   **Syntaxes :**

    -   **Sugared Syntax (forme courte, la plus lisible):**

        ```rust
        fn afficher<T: Afficher>(item: T) {
            item.afficher();
        }
        ```

        > ✅ Utilisée dans la majorité des cas simples

    -   **Syntaxe explicite avec `generic type`:**

        ```rust
        fn afficher(item: impl Afficher) {
            item.afficher();
        }
        ```

        > ✅ Plus concise pour les petits cas
        > ❌ Mais **non généralisable** à plusieurs paramètres ayant le même type

    -   **Syntaxe avec `where` (pour lisibilité dans les cas complexes):**

        ```rust
        fn traiter<T, U>(x: T, y: U)
        where
            T: Afficher + Clone,
            U: std::fmt::Debug,
        {
            x.afficher();
            println!("{:?}", y);
        }
        ```

        > ✅ Utile quand plusieurs paramètres génériques avec plusieurs contraintes

    | Syntaxe                | Utilisation idéale                               |
    | ---------------------- | ------------------------------------------------ |
    | `fn f<T: Trait>(x: T)` | ✅ Simple et claire pour une contrainte          |
    | `fn f(x: impl Trait)`  | ✅ Concis pour les fonctions à un seul paramètre |
    | `where T: Trait`       | ✅ Lisible pour plusieurs types ou contraintes   |

-   **Exemple complet et concret:**

    -   Contexte : Tu veux créer une fonction `sauvegarder_avec_log` qui :

        -   prend un type générique `T`
        -   **doit pouvoir être affiché (`Display`)**
        -   et **converti en JSON (`Serialize`)**

    ```rust
    use std::fmt::Display;
    use serde::Serialize; // Trait fourni par la crate `serde`

    fn sauvegarder_avec_log<T>(valeur: &T)
    where
        T: Display + Serialize,
    {
        println!("Valeur à sauvegarder : {}", valeur);

        // Convertir en JSON
        let json = serde_json::to_string(valeur).unwrap();
        println!("Sauvegarde JSON : {}", json);

        // Simuler sauvegarde
        // save_to_db(json);
    }
    ```

    ```rust
    #[derive(Serialize)]
    struct Client {
        nom: String,
        age: u32,
    }

    impl std::fmt::Display for Client {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ({})", self.nom, self.age)
        }
    }

    fn main() {
        let client = Client { nom: "Alice".into(), age: 30 };
        sauvegarder_avec_log(&client);
    }
    ```

## 3. Retourner un type implémentant un `trait`

-   **Définition:**

    > Tu peux faire en sorte qu’une fonction retourne **un type qui implémente un trait donné**, **sans exposer le type concret**.

-   **Syntaxe : `-> impl Trait`**

    ```rust
    trait Afficher {
        fn afficher(&self);
    }

    struct Client;

    impl Afficher for Client {
        fn afficher(&self) {
            println!("Je suis un client.");
        }
    }

    // 🔽 Retourne un type qui implémente Afficher
    fn creer_affichable() -> impl Afficher {
        Client
    }
    ```

-   Limitation : un **seul type concret** possible par fonction

    ```rust
    // ❌ CE CODE NE COMPILE PAS
    fn test(cond: bool) -> impl Afficher {
        if cond {
            Client
        } else {
            Produit // ← différent type implémentant Afficher
        }
    }
    ```

    > Si tu veux retourner **plusieurs types possibles**, il faut passer par un **trait object** avec `Box<dyn Trait>` (ce sera vu dans une prochaine section).

## 4. **implémentation conditionnelle**

-   **Définition:**

    > Rust permet d’implémenter des **méthodes uniquement si un type générique implémente un trait donné**.

    > C’est ce qu’on appelle une **implémentation conditionnelle**.

-   **Syntaxe avec `where`:**

    ```rust
    struct Wrapper<T> {
        valeur: T,
    }

    impl<T> Wrapper<T> {
        // Méthode toujours disponible
        fn new(valeur: T) -> Self {
            Wrapper { valeur }
        }
    }

    // Implémentation conditionnelle : uniquement si T implémente Display
    use std::fmt::Display;

    impl<T> Wrapper<T>
    where
        T: Display,
    {
        fn afficher(&self) {
            println!("Valeur : {}", self.valeur);
        }
    }
    ```

-   **Exemple :**

    ```rust
    fn main() {
        let w1 = Wrapper::new(42);        // i32 implémente Display → OK
        w1.afficher();

        let w2 = Wrapper::new(vec![1, 2]); // Vec<i32> implémente Display → OK
        w2.afficher();

        // let w3 = Wrapper::new(some_struct_sans_display); // ❌ Pas d'afficher()
    }
    ```

## 5. **Blanket Implementation:**

-   **Définition:**

    > Une **Blanket Implementation** est une **implémentation générique d’un trait** pour **tous les types qui satisfont une certaine contrainte**.

    > ie : _"implémenter un trait pour **tous les types** qui implémentent **un autre trait**."_

-   **Syntaxe:**

    ```rs
    impl<T : Display> NomTrait for T {
        // trait impl
    }

    // ou

    impl<T : Display> NomTrait for NomStruct<T> {
        // trait impl
    }
    ```

    ```rust
    trait MonTrait {
        fn action(&self);
    }

    impl<T: AutreTrait> MonTrait for T {
        fn action(&self) {
            println!("T implémente déjà AutreTrait → donc aussi MonTrait");
        }
    }
    ```

-   **Exemple typique :**

    Rust fournit déjà cette blanket impl :

    ```rust
    impl<T: ?Sized + Display> ToString for T {
        fn to_string(&self) -> String {
            let mut buf = String::new();
            std::fmt::write(&mut buf, format_args!("{}", self)).unwrap();
            buf
        }
    }
    ```

    > "Tout type `T` qui implémente `Display` implémente automatiquement `ToString`."

-   **Exemple :**

    > Objectif : Tous les types qui implémentent `Display` auront aussi une méthode `log()` via un nouveau trait.

    ```rust
    use std::fmt::Display;

    // Nouveau trait que nous voulons "propager"
    trait Loggable {
        fn log(&self);
    }

    // Blanket impl : pour tous les T qui implémentent Display
    impl<T: Display> Loggable for T {
        fn log(&self) {
            println!("[LOG] {}", self);
        }
    }

    // Testons sur un type standard
    fn main() {
        let x = 42;
        let name = "Alice";

        x.log();       // i32 implémente Display → OK
        name.log();    // &str aussi → OK
    }
    ```

    -   > ✅ Aucun besoin d’implémenter `Loggable` pour chaque type manuellement.
    -   > ✅ On **étend le comportement** de façon **automatique et générique**.
