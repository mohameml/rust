# cour 04 : **Fonctions**

## 1. **Introduction aux Fonctions en Rust:**

-   **Définition :**

    > Une **fonction** en Rust est un bloc de code réutilisable, défini avec le mot-clé `fn`, qui peut **prendre des paramètres** et **retourner une valeur**.

-   **Syntaxe générale** :

    ```rust
    fn nom_fonction(param1: Type1, param2: Type2) -> ReturnType {
        // corps de la fonction

        // final_value
        // or
        // return final_value
    }
    ```

-   **Exemple simple** :

    ```rust
    fn main() {
        dire_bonjour();
    }

    fn dire_bonjour() {
        println!("Bonjour !");
    }
    ```

    > 💡 En Rust, le point d'entrée du programme est toujours la fonction `main`.

## 2. **Statements et Expressions**

-   Rust est un **langage basé sur les expressions**. Comprendre la différence entre **statements** (instructions) et **expressions** (valeurs évaluées) est essentiel, notamment pour :

    -   comprendre le **corps des fonctions**,
    -   maîtriser la **valeur de retour implicite**,
    -   éviter des **erreurs de compilation** courantes.

### 2.1. **Définition : Statement**

-   Un **statement** est une instruction qui **fait une action** mais **ne retourne pas de valeur**.

-   **Exemples :**

    ```rust
    let y = 6;            // Déclaration de variable : statement
    fn main() {}          // Définition de fonction : statement
    ```

    ```rust
    let x = (let y = 6);  // Erreur ! `let y = 6` est un statement, pas une expression.
    ```

    🧨 **Erreur Rust :**

    > `expected expression, found let statement`

-   Contrairement à C ou Ruby, Rust **ne permet pas les assignations imbriquées** (ex: `x = y = 6`).

### 2.2. **Définition : Expression**

-   Une **expression** produit **une valeur**.

-   Elle peut être utilisée **dans une assignation**, ou être **le retour implicite** d’un bloc ou d’une fonction.

-   **Exemples :**

    ```rust
    5 + 6               // Expression : évalue à 11
    "Hello"             // Expression : évalue à une string
    fonction(3)         // Appel de fonction : expression
    {
        let x = 2;
        x + 1           // expression retournée = 3
    }                   // bloc = expression entière évaluée à 3
    ```

-   ⚠️ **Les expressions ne se terminent pas par `;`** !

-   Si tu mets un `;`, tu transformes l’expression en statement.

### 2.3. **Exemples pratiques**

-   **Expression dans un bloc :**

    ```rust
    fn main() {
        let y = {
            let x = 3;
            x + 1  // dernière expression du bloc, pas de `;`
        };
        println!("y = {y}"); // affiche y = 4
    }
    ```

-   **❌ Erreur si `;` ajouté :**

    ```rust
    let y = {
        let x = 3;
        x + 1; // <- point-virgule = statement ⇒ retourne ()
    };
    ```

    ➡️ Ici, `y` vaudrait `()` (le type `unit`), pas `4`.

## 3. **Retour Implicite et Explicite en Rust**

> Rust permet deux manières de **retourner une valeur depuis une fonction** : `Retour Implicite et Explicite`

### 3.1. **Retour **explicite** : avec `return`**

-   **Définition :**

    > Le **retour explicite** utilise le mot-clé `return` pour indiquer **clairement** la valeur renvoyée.

-   **Syntaxe :**

    ```rust
    fn nom_fonction(...) -> Type {
        return valeur;
    }
    ```

-   **Exemple :**

    ```rust
    fn carre(x: i32) -> i32 {
        return x * x;
    }

    fn main() {
        println!("{}", carre(3)); // Affiche 9
    }
    ```

### 3.2. **Retour **implicite** : sans `return`, sans `;`**

-   **Définition :**

    Le **retour implicite** utilise la **dernière expression d'une fonction** (ou d’un bloc) **sans point-virgule `;`** pour la retourner automatiquement.

-   **Syntaxe :**

    ```rust
    fn nom_fonction(...) -> Type {
        expression_sans_point_virgule
    }
    ```

-   **Exemple :**

    ```rust
    fn carre(x: i32) -> i32 {
        x * x // pas de `;` → retour implicite
    }

    fn main() {
        println!("{}", carre(4)); // Affiche 16
    }
    ```

-   **⚠️ Attention à ne pas mettre de `;`**

    ```rust
    fn carre(x: i32) -> i32 {
        x * x; // ❌ avec `;`, retourne () (type unit), erreur de compilation
    }
    ```
