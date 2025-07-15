# Cour : **Vecteur :**

## 1. **Définition et création d'un vecteur:**

-   **Définition:**

    > Un vecteur (`Vec<T>`) est une collection **dynamique** d'éléments du même type `T`. Contrairement aux tableaux de taille fixe `[T; N]`, les vecteurs peuvent **croître ou rétrécir** dynamiquement.

    > Type générique : `Vec<T>` signifie "vecteur contenant des éléments de type T".

-   **`Vec::new()`:**

    ```rust
    let v: Vec<i32> = Vec::new();  // Vecteur vide de i32
    ```

-   **`vec![]` (macro):**

    ```rust
    let v = vec![1, 2, 3];  // Crée un vecteur avec 3 entiers
    ```

-   **`Vec::with_capacity(n)`:**

    Crée un vecteur avec une **capacité initiale**, sans éléments :

    ```rust
    let mut v = Vec::with_capacity(10);
    ```

## 2. **Accès et itération:**

### 2.1 **Accès aux éléments:**

-   Par index (attention à `panic!` si hors bornes)

    ```rust
    let v = vec![10, 20, 30];
    println!("{}", v[1]); // Affiche 20
    ```

-   **Par `get()` (renvoie `Option<T>`):\***

    ```rust
    if let Some(x) = v.get(2) {
        println!("Valeur: {}", x);
    }
    ```

### 2.2 **Itération:**

-   **Lecture seule (`&`):**

    ```rust
    let v = vec![1, 2, 3];
    for x in &v {
        println!("{}", x);
    }
    ```

-   **Lecture / écriture (`&mut`):**

    ```rust
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x += 10;
    }
    ```

-   **Consommation (prend possession):**

    ```rust
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}", x); // v ne peut plus être utilisé après
    }
    ```

## 3. **Tableau des méthodes principales de `Vec<T>`:**

| Méthode         | Syntaxe                             | Description                                                 |
| --------------- | ----------------------------------- | ----------------------------------------------------------- |
| `new`           | `Vec::new()`                        | Crée un vecteur vide                                        |
| `with_capacity` | `Vec::with_capacity(n)`             | Crée un vecteur vide avec une capacité de `n` éléments      |
| `push`          | `v.push(val)`                       | Ajoute un élément à la fin                                  |
| `pop`           | `v.pop()`                           | Supprime et retourne le dernier élément (`Option<T>`)       |
| `get`           | `v.get(i)`                          | Accès sécurisé à l'élément à l'indice `i` (`Option<&T>`)    |
| `remove`        | `v.remove(i)`                       | Retire l’élément à l’indice `i` (panique si hors bornes)    |
| `insert`        | `v.insert(i, val)`                  | Insère `val` à l'indice `i`                                 |
| `len`           | `v.len()`                           | Renvoie le nombre d’éléments                                |
| `capacity`      | `v.capacity()`                      | Renvoie la capacité allouée                                 |
| `clear`         | `v.clear()`                         | Vide complètement le vecteur                                |
| `is_empty`      | `v.is_empty()`                      | Vérifie si le vecteur est vide                              |
| `resize`        | `v.resize(n, val)`                  | Redimensionne le vecteur à `n` éléments, remplit avec `val` |
| `truncate`      | `v.truncate(n)`                     | Garde seulement les `n` premiers éléments                   |
| `extend`        | `v.extend(iter)`                    | Étend le vecteur avec un itérateur                          |
| `iter`          | `v.iter()`                          | Itérateur immuable                                          |
| `iter_mut`      | `v.iter_mut()`                      | Itérateur mutable                                           |
| `into_iter`     | `v.into_iter()`                     | Itérateur de consommation (déplace la propriété)            |
| `retain`        | \`v.retain(\|x\| condition)\`       | Garde seulement les éléments qui satisfont la condition     |
| `sort`          | `v.sort()`                          | Trie le vecteur (si `T: Ord`)                               |
| `sort_by`       | \`v.sort_by( \| a, b \| a.cmp(b))\` | Trie avec fonction personnalisée                            |
