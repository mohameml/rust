# Cour : **Testing**

## 1. Introduction au testing en Rust

-   **Définition:**

    > Le testing est le processus de vérification automatique que ton code fonctionne comme prévu.

    -   Il permet de :

        -   détecter des **bugs** plus tôt,
        -   faciliter le **refactoring**,
        -   garantir la **qualité** du logiciel.

-   **Comment ça marche en Rust ?:**

    Rust possède un framework de test intégré dans le compilateur via le module `test`.

    -   `#[cfg(test)]` : indique que le bloc de code doit **seulement être compilé pendant les tests**.
    -   `#[test]` : marque une **fonction de test**.

-   **Types de tests:**

    Rust distingue plusieurs **états de test** :

    | Type                | Signification                                                                            |
    | ------------------- | ---------------------------------------------------------------------------------------- |
    | ✅ **Passed**       | Le test a réussi                                                                         |
    | ❌ **Failed**       | Le test a échoué                                                                         |
    | 🚫 **Ignored**      | Le test a été marqué pour être ignoré (`#[ignore]`)                                      |
    | ⏱️ **Measured**     | Le temps d'exécution est mesuré (en mode benchmark, pas encore stable)                   |
    | 🕵️ **Filtered Out** | Le test ne correspond pas aux filtres passés en ligne de commande (`cargo test my_test`) |

-   **Exemple basique :**

    ```rust
    // Module de test privé
    #[cfg(test)]
    mod tests {
        use super::*; // pour accéder au reste du code

        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }
    ```

## 2. Macros de test principales

| Macro              | Description                                | Exemple                                 |
| ------------------ | ------------------------------------------ | --------------------------------------- |
| `assert!`          | Vérifie si une condition est vraie         | `assert!(x > 0, "x doit être positif")` |
| `assert_eq!`       | Vérifie que deux valeurs sont égales       | `assert_eq!(2 + 2, 4)`                  |
| `assert_ne!`       | Vérifie que deux valeurs sont différentes  | `assert_ne!(a, b)`                      |
| `debug_assert!`    | Comme `assert!`, mais désactivé en release | `debug_assert!(x > 0)`                  |
| `debug_assert_eq!` | Comme `assert_eq!`, désactivé en release   | `debug_assert_eq!(x, y)`                |
| `debug_assert_ne!` | Comme `assert_ne!`, désactivé en release   | `debug_assert_ne!(a, b)`                |

## 3. **Tests qui retournent `Result`:**

-   **Intro:**

    > Depuis Rust 1.26, tu peux écrire des fonctions de test qui retournent `Result<(), E>`, ce qui permet d'utiliser `?` pour propager les erreurs.

    -   Habituellement, les tests utilisent :

        ```rust
        #[test]
        fn test_truc() {
            assert_eq!(2 + 2, 4);
        }
        ```

    -   Mais **depuis Rust 1.26**, on peut écrire :

        ```rust
        #[test]
        fn test_truc() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("Le test a échoué"))
            }
        }
        ```

    > Plus lisible et idiomatique que `assert!`.

-   Avantages d’utiliser `Result`

    | Avantage                             | Explication                                                                           |
    | ------------------------------------ | ------------------------------------------------------------------------------------- |
    | ✅ Utiliser `?`                      | Tu peux propager une erreur simplement avec `?`, comme dans une fonction normale.     |
    | ✅ Plus lisible                      | Pas besoin de `assert!` partout, juste retourner `Err(...)`.                          |
    | ✅ Intégré au système d’erreurs Rust | Tu peux tester des fonctions qui retournent déjà des `Result` de manière idiomatique. |

-   **Exemple 1 : test simple avec Result**

    ```rust
    #[test]
    fn test_result_succes() -> Result<(), String> {
        let x = 2 + 2;
        if x == 4 {
            Ok(())
        } else {
            Err(format!("Mauvais résultat : {}", x))
        }
    }
    ```

-   **Exemple 2 : utiliser `?` pour propager une erreur**

    ```rust
    fn parse_age(age_str: &str) -> Result<u32, std::num::ParseIntError> {
        age_str.parse()
    }

    #[test]
    fn test_parse_age() -> Result<(), Box<dyn std::error::Error>> {
        let age = parse_age("42")?;
        assert_eq!(age, 42);
        Ok(())
    }
    ```

    Ici :

    -   Si `parse_age("42")` échoue, le test retourne immédiatement l'erreur.
    -   Sinon, on continue et on vérifie que `age == 42`.

## 4. **Attributs de test en Rust:**

-   **Déf:**

    > Les attributs sont des **annotations** qui modifient le comportement des tests.

-   **`#[test]`:**

    > Marque une fonction comme **fonction de test**.

    ```rust
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
    ```

-   **`#[should_panic(expected = "msg")]`**

    > Indique que le test est **réussi s’il panique**.

    Utile pour tester les cas où une erreur est attendue.

    ```rust
    #[test]
    #[should_panic(expected = "division by zero")]
    fn division_par_zero() {
        let _ = 1 / 0;
    }
    ```

    -   Tu peux spécifier un message d’erreur attendu avec `expected = "..."`
    -   Si la fonction **ne panique pas**, le test échoue.

-   **`#[ignore]`:**

    > Le test sera **ignoré par défaut** sauf si on exécute `cargo test -- --ignored`.

    ```rust
    #[test]
    #[ignore]
    fn test_lent() {
        // Long test qu'on veut ignorer par défaut
    }
    ```

-   **`#[cfg(test)]`:**

    > Compilation conditionnelle : le code annoté est **inclus uniquement en mode test**.

    Typiquement utilisé pour créer un module de test :

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn exemple() {
            assert!(true);
        }
    }
    ```

    -   Le bloc n’est **pas compilé en production**.
    -   Cela permet d’avoir des fonctions et imports spécifiques aux tests sans polluer le reste du code.

## 5. Tests d’intégration (`tests/`)

-   **Définition:**

    > Les **tests d’intégration** testent ton **crate comme un tout**. Contrairement aux tests unitaires (dans le même fichier que le code testé), ils sont écrits dans le dossier `tests/` à la racine du projet.

    -   Ils utilisent l'API publique du crate, comme le ferait un utilisateur externe.

-   **Structure:**

    ```
    mon_crate/
    ├── src/
    │   └── lib.rs
    ├── tests/
    │   ├── integration_test.rs
    │   └── autre_test.rs
    ```

-   **Exemple : `tests/integration_test.rs`**

    ```rust
    use mon_crate::add; // Fonction publique du lib.rs

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    ```

    > ⚠️ Il faut que la fonction `add` soit `pub`.

    ```bash
    cargo test
    ```

    Ils sont automatiquement détectés si le fichier est dans `tests/`.

## 6. **Commandes avancées avec `cargo test`:**

-   **Définition:**

    > `cargo test` compile ton projet en mode test, exécute tous les tests et affiche les résultats.

-   **Commandes utiles:**

    | Commande                         | Description                                          |
    | -------------------------------- | ---------------------------------------------------- |
    | `cargo test`                     | Lance tous les tests (unitaires + intégration).      |
    | `cargo test nom_du_test`         | Filtre par nom de test.                              |
    | `cargo test -- --nocapture`      | Affiche les `println!()` dans les tests.             |
    | `cargo test -- --ignored`        | Exécute les tests marqués avec `#[ignore]`.          |
    | `cargo test -- --test-threads=1` | Lance les tests **séquentiellement**.                |
    | `cargo test --release`           | Exécute les tests **en mode release** (plus rapide). |

-   **Exemple:**

    ```bash
    cargo test add -- --nocapture
    ```

    -   Exécute uniquement les tests qui contiennent "add"
    -   Affiche les messages `println!`
