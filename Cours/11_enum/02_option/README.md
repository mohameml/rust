# Cour : **Option:**

## 1. **Introduction:**

-   **Définition:**

    > `Option` est une énumération standard de Rust qui représente un type pouvant soit contenir une valeur (`Some`), soit ne rien contenir (`None`). C'est la manière idiomatique en Rust de gérer les cas où une valeur peut être absente, évitant ainsi le problème des références nulles présent dans d'autres langages.

-   **Syntaxe:**

    L'enum `Option` est définie dans la bibliothèque standard comme suit :

    ```rust
    pub enum Option<T> {
        None,
        Some(T),
    }
    ```

    Où :

    -   `T` est un type générique
    -   `Some(T)` représente une valeur présente de type `T`
    -   `None` représente l'absence de valeur

-   **Exemple :**

    ```rust
    fn trouver_index(v: Vec<i32>, valeur: i32) -> Option<usize> {
        for (index, &element) in v.iter().enumerate() {
            if element == valeur {
                return Some(index);
            }
        }
        None
    }

    let nombres = vec![10, 20, 30];
    match trouver_index(nombres, 20) {
        Some(index) => println!("Trouvé à l'index {}", index),
        None => println!("Valeur non trouvée"),
    }
    ```

## 2. **Méthodes:**

| Méthode              | Exemple                                      | Description courte                                      |
| -------------------- | -------------------------------------------- | ------------------------------------------------------- |
| `.is_some()`         | `opt.is_some()`                              | Vrai si l'option contient une valeur (`Some`).          |
| `.is_none()`         | `opt.is_none()`                              | Vrai si l'option est vide (`None`).                     |
| `.unwrap()`          | `opt.unwrap()`                               | Récupère la valeur, panique si `None`.                  |
| `.unwrap_or(x)`      | `opt.unwrap_or(10)`                          | Renvoie la valeur ou `10` si `None`.                    |
| `.unwrap_or_else(f)` | `opt.unwrap_or_else(\|\| compute_default())` | Renvoie la valeur ou appelle une fonction de secours.   |
| `.map(f)`            | `opt.map(\|x\| x + 1)`                       | Applique une fonction si `Some`, sinon retourne `None`. |
| `.and_then(f)`       | `opt.and_then(\|x\| Some(x + 1))`            | Chaîne des `Option`, permet des opérations imbriquées.  |
| `.filter(f)`         | `opt.filter(\|x\| *x > 0)`                   | Garde la valeur si elle respecte un prédicat.           |
| `.or(opt2)`          | `opt.or(Some(5))`                            | Renvoie `opt` si `Some`, sinon `opt2`.                  |
| `.or_else(f)`        | `opt.or_else(\|\| Some(5))`                  | Comme `or` mais calcule `opt2` à la volée.              |
| `.ok_or(err)`        | `opt.ok_or("erreur")`                        | Transforme `Option` en `Result`, `None` devient `Err`.  |
| `.ok_or_else(f)`     | `opt.ok_or_else(\|\| "erreur")`              | Comme `ok_or` mais avec fonction de secours.            |
| `.as_ref()`          | `opt.as_ref()`                               | Donne une référence vers la valeur contenue.            |
| `.as_mut()`          | `opt.as_mut()`                               | Donne une référence mutable vers la valeur.             |
| `.take()`            | `opt.take()`                                 | Vide l’option et renvoie l’ancienne valeur.             |
| `.replace(val)`      | `opt.replace(42)`                            | Remplace la valeur, renvoie l’ancienne.                 |
| `.expect(msg)`       | `opt.expect("doit exister")`                 | Comme `unwrap` mais avec message d’erreur personnalisé. |

-   **Exemple 1 : Chaînage d'options**

    ```rust
    fn carre(x: i32) -> Option<i32> {
        Some(x * x)
    }

    fn racine_carree(x: i32) -> Option<f64> {
        if x >= 0 {
            Some((x as f64).sqrt())
        } else {
            None
        }
    }

    let result = Some(4)
        .and_then(carre)
        .and_then(|x| racine_carree(x));

    println!("{:?}", result); // Some(4.0)
    ```

-   **Exemple 2 : Méthodes utiles d'Option**

    ```rust
    let x = Some(10);
    let y: Option<i32> = None;

    // unwrap (à utiliser avec précaution)
    println!("{}", x.unwrap()); // 10
    // println!("{}", y.unwrap()); // Panique !

    // unwrap_or
    println!("{}", x.unwrap_or(0)); // 10
    println!("{}", y.unwrap_or(0)); // 0

    // map
    let doubled = x.map(|n| n * 2);
    println!("{:?}", doubled); // Some(20)

    // filter
    let even = x.filter(|&n| n % 2 == 0);
    println!("{:?}", even); // Some(10)
    ```
