# Cour : **les itérateurs en Rust**

## 1. **Introduction aux itérateurs en Rust**

### 1.1 **Définition:**

> Un **itérateur** en Rust est **un objet** qui permet de **parcourir une séquence d’éléments** un par un.

-   Il implémente le **trait `Iterator`** :

    ```rust
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    ```

    -   Chaque appel à `next()` retourne :

        -   `Some(valeur)` s’il y a un élément suivant,
        -   `None` s’il n’y a plus rien.

-   Rust a une **approche paresseuse** (_lazy_), donc les itérateurs **ne font rien** tant qu’on ne **les consomme pas** (par ex. avec `for`, `collect`, etc.).

### 1.2 **Méthodes d’itération : `iter`, `into_iter`, `iter_mut`**

-   **`iter()`:**

    -   🔒 Emprunt **immuable** des éléments.
    -   Retourne un itérateur sur des **références**.
    -   Ne consomme pas la collection.

    ```rust
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x); // &i32
    }
    ```

-   **`iter_mut()`:**

    -   🔁 Emprunt **mutable** des éléments.
    -   Permet de **modifier** les éléments en place.

    ```rust
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", v); // [2, 4, 6]
    ```

-   **`into_iter()`:**

    -   Consomme la collection.
    -   Retourne les **valeurs directement** (pas des références).
    -   Très utilisé avec `for`.

    ```rust
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x); // i32
    }
    // v n'est plus accessible ici
    ```

-   **Différences clés entre `iter`, `iter_mut`, `into_iter`:**

    | Méthode       | Accès aux éléments | Références ou valeurs | Consomme la collection ? |
    | ------------- | ------------------ | --------------------- | ------------------------ |
    | `iter()`      | Lecture            | `&T`                  | ❌ Non                   |
    | `iter_mut()`  | Écriture           | `&mut T`              | ❌ Non                   |
    | `into_iter()` | Possession         | `T`                   | ✅ Oui                   |

### RQ : **Lien entre `for` et les itérateurs:**

La boucle `for` en Rust est **un sucre syntaxique** pour un appel à `.into_iter()` :

```rust
let v = vec![1, 2, 3];
for x in v {
    println!("{}", x);
}
```

Est équivalent à :

```rust
let mut it = v.into_iter();
while let Some(x) = it.next() {
    println!("{}", x);
}
```

➡ `for` appelle implicitement `.into_iter()` sur ce qu’on lui passe.

## 2. **le mot-clé `type`**

-   **Définition:**

> Le mot-clé `type` en Rust sert à **introduire un alias de type** ou à **spécifier un type associé** dans un trait.

-   Il est utilisé dans deux contextes principaux : **Type associé** à un trait et **Type alias**

### 2.1 **Type associé** à un trait

-   **Définition:**

    > Un **type associé** est une **déclaration abstraite de type** dans un trait, que l’implémentation devra concrétiser.

    -   C’est une **alternative aux génériques**.

-   **Syntaxe:**

    ```rust
    pub trait Iterator {
        type Item; // <- type associé

        fn next(&mut self) -> Option<Self::Item>;
    }
    ```

-   **Exemple 1:**

    Chaque type qui implémente `Iterator` doit **spécifier le type d’élément** produit :

    ```rust
    impl Iterator for std::ops::Range<u32> {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            // logique...
        }
    }
    ```

    ✔ Ici, `type Item = u32;` signifie que cet itérateur retourne des `u32`.

-   **Exemple 2:**

    ```rust
    fn afficher_iter<I: Iterator>(mut iter: I)
    where
        I::Item: std::fmt::Debug,
    {
        while let Some(val) = iter.next() {
            println!("{:?}", val);
        }
    }
    ```

    ➡ Ici, `I::Item` est le **type associé** défini par le trait `Iterator`.

### 2.2 **Type alias :**

-   **Définition:**

    > Un **type alias** permet de **renommer** un type complexe, pour des raisons de lisibilité ou de simplification.

-   **Syntaxe:**

    ```rust
    type MonAlias = u64;
    ```

-   **Exemple:**

    ```rust
    type Point = (i32, i32);

    fn distance(p1: Point, p2: Point) -> f64 {
        let dx = (p2.0 - p1.0) as f64;
        let dy = (p2.1 - p1.1) as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
    ```

    ➡ On peut aussi aliaser des closures, des types de fonctions, ou même des types paramétrés :

    ```rust
    type Transform = fn(i32) -> i32;
    ```

## 3. **trait `Iterator`:**

### 3.1. **Le trait `Iterator`:**

-   **Définition:**

    Le **trait `Iterator`** est au coeur du système d’itération en Rust. Il est défini comme suit :

    ```rust
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    ```

-   **syntaxe:**

    ```rust
    struct MyStruct;

    impl Iterator for MyStruct {
        type Item = MyType;

        fn next(&mut self) -> Option<Self::Item> {
            // logique d'itération
        }
    }
    ```

### 3.2. **Le trait `IntoIterator`:**

-   **Définition:**

    Le trait `IntoIterator` est utilisé pour **transformer un objet en itérateur**.

    ```rust
    pub trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }
    ```

### 3.3. **Implémentation d’un itérateur personnalisé:**

> Créer notre **propre structure** qui peut être utilisée avec une boucle `for` ou manuellement avec `.next()`.

-   **Exemple:**

    -   **struct:**

        ```rust
        struct Counter {
            current: u32,
            max: u32,
        }
        ```

    -   **iterator:**

        ```rust
        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current <= self.max {
                    let res = self.current;
                    self.current += 1;
                    Some(res)
                } else {
                    None
                }
            }
        }
        ```

    -   **intro_iterator:**

        ```rust
        impl IntoIterator for Counter {
            type Item = u32;
            type IntoIter = Self;

            fn into_iter(self) -> Self::IntoIter {
                self
            }
        }
        ```

    -   \*\*main

    ```rust
    fn main() {
        let counter = Counter { current: 1, max: 5 };

        for i in counter {
            println!("{}", i);  // Affiche 1 à 5
        }
    }
    ```

---

## 🧠 À retenir

-   `Iterator` = comportement de l’itérateur (comment produire les éléments).
-   `IntoIterator` = conversion d’un objet en itérateur (utilisé dans les boucles `for`).
-   Implémenter ces deux traits vous permet de **créer des structures itérables personnalisées** très puissantes.

---

Souhaites-tu qu’on continue avec une **Section 4** sur les **méthodes adaptatrices des itérateurs** (`map`, `filter`, `take`, `enumerate`, `collect`, etc.) ?
