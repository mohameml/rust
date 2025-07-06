# 📝 Énoncé du mini-projet : `smart-calc`

## 1. **Introduction:**

-   Développer en Rust une **calculatrice intelligente** en ligne de commande capable de :

    -   **Analyser une expression mathématique** saisie par l'utilisateur.
    -   **Évaluer** cette expression avec :

        -   opérations de base : `+`, `-`, `*`, `/`, `^`
        -   parenthèses imbriquées
        -   fonctions mathématiques : `sin(x)`, `cos(x)`, `tan(x)`, `sqrt(x)`, `log(x)`

    -   **Gérer les erreurs de syntaxe** avec des **messages explicites**.
    -   Retourner le **résultat numérique** de l'expression si elle est correcte.

> Exemple d’entrée valide :
>
> ```
> (1+2)*2 + 3*cos(30) + sqrt(10)*2 - 10
> ```
>
> Résultat :
>
> ```
> Résultat : 5.7963
> ```

> Exemple d’entrée invalide :
>
> ```
> 3 + * 2
> ```
>
> Résultat :
>
> ```
> Erreur de syntaxe : opérateur `*` inattendu après `+`
> ```

## 2. Plan du projet

### 2.1. **Initialisation du projet**

-   Créer un projet Rust :

    ```bash
    cargo new smart-calc
    ```

### 2.2 **Parsing de l’expression**

-   Lire l’entrée de l’utilisateur (`stdin`).
-   Nettoyer l’entrée : supprimer les espaces inutiles.

### 2.3. **Tokenizer**

-   Convertir l’expression en **tokens** :

    -   nombres (`42`, `3.14`)
    -   opérateurs (`+`, `-`, etc.)
    -   parenthèses
    -   fonctions (`sin`, `cos`, etc.)

### 2.4. **Analyse syntaxique (parser)**

-   Construire un **arbre syntaxique (AST)**.
-   Implémenter une grammaire simple (shunting-yard ou recursive descent).
-   Gérer :

    -   priorité des opérateurs
    -   associativité
    -   fonctions mathématiques

### 2.5. **Évaluation de l’AST**

-   Parcourir récursivement l’AST pour calculer la valeur de l'expression.

### 2.6. **Gestion des erreurs**

-   Détecter :

    -   parenthèses non fermées
    -   opérateurs mal placés
    -   noms de fonctions inconnus

-   Fournir des messages d’erreur explicites (avec position si possible).

### 2.7. **Fonctions mathématiques**

-   Utiliser `libm` ou `f64` pour :

    -   `sin(x)`, `cos(x)`, `tan(x)`, `sqrt(x)`, `log(x)`, etc.

-   Attention aux unités : préciser si `cos(30)` est en degrés ou radians.

### 2.8. **Interface utilisateur**

-   Afficher une invite (`>`) en boucle (REPL).
-   Afficher le résultat ou l’erreur.

## 3. Bonus possibles

-   Support des **variables** : `x = 2`, puis `x + 3`
-   Support de **constantes** : `pi`, `e`
-   Mode **degré/radian** toggle
-   Coloration syntaxique des erreurs

## 4. Structure de fichiers suggérée

```
smart-calc/
│
├── src/
│   ├── main.rs         # Entrée du programme, boucle REPL
│   ├── lexer.rs        # Tokenisation
│   ├── parser.rs       # AST + grammaire
│   ├── evaluator.rs    # Évaluation de l'AST
│   └── error.rs        # Définition des erreurs syntaxiques
```
