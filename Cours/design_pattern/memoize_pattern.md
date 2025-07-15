Très bonne transition ! Parlons maintenant du **pattern de mémoïsation** en Rust.

---

## 🧠 Mémoïsation en Rust

### 📌 Définition

La **mémoïsation** est une technique d’optimisation qui consiste à **mémoriser les résultats de fonctions coûteuses** pour ne pas les recalculer lorsqu’elles sont appelées à nouveau avec les **mêmes arguments**.

---

### 🛠 Objectif

Créer une **fonction ou une closure mémoïsée**, qui :

-   calcule le résultat une fois,
-   **stocke** ce résultat,
-   puis le **renvoie directement** la fois suivante sans recalcul.

---

### 🧱 Exemple simple : Fibonacci sans mémoïsation

```rust
fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

➡ Inefficace car **recalcule** les mêmes valeurs plusieurs fois.

---

## ✅ Mémoïsation avec une `HashMap`

### 📦 Implémentation manuelle

```rust
use std::collections::HashMap;

fn fib(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if let Some(&val) = cache.get(&n) {
        return val;
    }

    let result = if n <= 1 {
        n
    } else {
        fib(n - 1, cache) + fib(n - 2, cache)
    };

    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    let result = fib(40, &mut cache);
    println!("fib(40) = {}", result);
}
```

✔ Ici :

-   Le `cache` est passé par référence.
-   On évite de recalculer `fib(n)` s’il est déjà dans le cache.

---

## 🔁 Mémoïsation avec une **closure** mémoïsée

On peut généraliser en encapsulant la logique de cache dans une structure :

### 🧰 Struct `Memoizer`

```rust
use std::collections::HashMap;

struct Memoizer<T, U>
where
    T: Eq + std::hash::Hash + Copy,
    U: Copy,
{
    func: Box<dyn Fn(T) -> U>,
    cache: HashMap<T, U>,
}

impl<T, U> Memoizer<T, U>
where
    T: Eq + std::hash::Hash + Copy,
    U: Copy,
{
    fn new(func: impl Fn(T) -> U + 'static) -> Self {
        Memoizer {
            func: Box::new(func),
            cache: HashMap::new(),
        }
    }

    fn call(&mut self, arg: T) -> U {
        if let Some(&val) = self.cache.get(&arg) {
            return val;
        }
        let val = (self.func)(arg);
        self.cache.insert(arg, val);
        val
    }
}
```

### 💡 Utilisation

```rust
fn main() {
    let mut fib_memo = Memoizer::new(|n: u32| {
        if n <= 1 {
            n
        } else {
            fib_memo.call(n - 1) + fib_memo.call(n - 2)
        }
    });

    // Appel
    let result = fib_memo.call(30);
    println!("fib(30) = {}", result);
}
```

🛑 Cela **ne compile pas encore** car `fib_memo` s'appelle **récursivement** dans sa propre définition. Pour cela, on doit utiliser des astuces comme **`RefCell`, `Rc` ou `once_cell`**, ou bien utiliser des **crates externes**.

---

## 🚀 Alternatives : Crates existantes

-   [`cached`](https://docs.rs/cached) : Crate qui propose des macros pour faire de la mémoïsation facilement.

### Exemple avec `cached` :

```rust
use cached::cached;

cached! {
    FIB;
    fn fib(n: u32) -> u32 = {
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }
}
```

✔ Super simple à utiliser, très utile en production.

---

Souhaites-tu un exemple de **mémoïsation thread-safe** ou des astuces pour faire une **closure récursive mémoïsée proprement** en Rust ?
