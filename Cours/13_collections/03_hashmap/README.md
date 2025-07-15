# Cour : **HashMap**

## 1. **Définition formelle d’un `HashMap`:**

-   **Définition (formelle & théorique) :**

    > Un **`HashMap<K, V>`** est une **table de hachage** implémentée dans la bibliothèque standard de Rust.

    > C’est une **structure de données clé-valeur**, où chaque **clé `K` est associée à une valeur `V`**, et l’accès à la valeur se fait via un **hachage** de la clé.

-   **Conditions :**

    -   Les clés doivent implémenter le trait `Eq` et `Hash` (détection d’égalité + hachage).
    -   L’ordre des éléments **n’est pas garanti**.
    -   Basé sur une **fonction de hachage** pour retrouver rapidement une valeur.

## 2. **Création et syntaxe de base en Rust:**

-   **Import nécessaire :**

    ```rust
    use std::collections::HashMap;
    ```

-   **Création :**

    | Méthode                | Exemple                                                                    |
    | ---------------------- | -------------------------------------------------------------------------- |
    | Création vide          | `let mut map = HashMap::new();`                                            |
    | Insertion              | `map.insert("clé", 42);`                                                   |
    | Création via `collect` | `let map: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();` |

-   **Accès / mise à jour :**

    ```rust
    if let Some(val) = map.get("clé") {
        println!("valeur = {}", val);
    }

    map.insert("clé", 100); // met à jour ou insère
    map.remove("clé");      // supprime une entrée
    ```

## 3. **Méthodes principales:**

| Méthode                 | Description courte                       | Syntaxe                          |
| ----------------------- | ---------------------------------------- | -------------------------------- |
| `new()`                 | Crée un `HashMap` vide                   | `HashMap::new()`                 |
| `insert(k, v)`          | Ajoute ou met à jour une entrée          | `map.insert("a", 1);`            |
| `get(&k)`               | Accède à la valeur associée à `k`        | `map.get("a")`                   |
| `remove(&k)`            | Supprime la clé `k` et sa valeur         | `map.remove("a");`               |
| `contains_key(&k)`      | Vérifie si la clé existe                 | `map.contains_key("a")`          |
| `len()`                 | Nombre de paires clé-valeur              | `map.len()`                      |
| `is_empty()`            | Vérifie si la map est vide               | `map.is_empty()`                 |
| `entry(k).or_insert(v)` | Insère si la clé n'existe pas déjà       | `map.entry("clé").or_insert(0);` |
| `iter()`                | Itération immuable sur paires `(k, v)`   | `for (k, v) in &map { ... }`     |
| `keys()` / `values()`   | Itère sur les clés ou valeurs uniquement | `for k in map.keys()`            |

## 4. Exemple **réaliste** : compteur de mots

```rust
use std::collections::HashMap;

fn main() {
    let texte = "le rust est rapide et le rust est sûr";
    let mut compteur = HashMap::new();

    for mot in texte.split_whitespace() {
        // Incrémente la valeur associée à chaque mot
        let count = compteur.entry(mot).or_insert(0);
        *count += 1;
    }

    for (mot, nb) in &compteur {
        println!("{} → {}", mot, nb);
    }
}
```

| Méthode              | Rôle dans l'exemple                          |
| -------------------- | -------------------------------------------- |
| `entry()`            | Accès sécurisé avec insertion conditionnelle |
| `or_insert(0)`       | Initialise à zéro si mot non encore vu       |
| `*count += 1`        | Incrémentation de la fréquence               |
| `split_whitespace()` | Découpe du texte en mots                     |
| `iter()`             | Itération sur les paires `(mot, nb)`         |
