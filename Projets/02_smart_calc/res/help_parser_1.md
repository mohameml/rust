## 📘 1. Rôle du Parser

Le **parser** prend ta **liste de tokens** comme :

```rust
[LParen, Int(1), Plus, Int(2), RParen, Multiply, Int(3)]
```

Et construit une **structure hiérarchique** représentant l’expression, souvent sous forme d’un **AST (Abstract Syntax Tree)**.

---

## 🧠 2. Grammaire à utiliser

Voici une **grammaire d'expression arithmétique** simple (notation BNF modifiée) :

```bnf
Expr        → Term (( "+" | "-" ) Term)*
Term        → Factor (( "*" | "/" ) Factor)*
Factor      → ( "+" | "-" )? Primary
Primary     → number
            | const
            | Function "(" Expr ")"
            | "(" Expr ")"

number      → [0-9]+(.[0-9]+)?
const       → "pi" | "e"
Function    → "cos" | "sin" | "ln" | "exp" | "sqrt"

```

---

## 🔍 3. Ce que chaque règle veut dire

| **Règle** | **Description**                                                      |
| --------- | -------------------------------------------------------------------- |
| `Expr`    | Addition ou soustraction entre des `Term`                            |
| `Term`    | Multiplication ou division entre des `Factor`                        |
| `Factor`  | Éléments de base (valeurs, fonctions, expressions entre parenthèses) |
| `Unary`   | Gère les signes `+` ou `-` avant les nombres ou fonctions            |
| `Primary` | Nombre, fonction, constante, ou expression entre parenthèses         |

---

## 🧩 4. Exemple concret

Pour l’entrée :

```txt
(1 + 2) * 3
```

### Étapes :

-   `Expr`: `(1 + 2) * 3` ⟶ contient une `*`, donc deux `Term`s
-   Chaque `Term` contient des `Factor`s
-   `(1 + 2)` est un `Primary` qui contient une sous-`Expr`
-   `3` est un `Primary`

### AST simplifié :

```
         *
       /   \
     +       3
   /   \
  1     2
```

### Exemple 2 :

> Exemple d'arbre de syntaxe pour `(1+2)*3 + cos(pi)`

```bnf
<expression>
├─ <term>
│  ├─ <factor>
│  │  ├─ '('
│  │  ├─ <expression>
│  │  │  ├─ <term> (<number> "1")
│  │  │  ├─ '+'
│  │  │  └─ <term> (<number> "2")
│  │  └─ ')'
│  ├─ '*'
│  └─ <factor> (<number> "3")
├─ '+'
└─ <term>
   └─ <factor>
      └─ <function>
         ├─ 'cos'
         ├─ '('
         ├─ <expression>
         │  └─ <term>
         │     └─ <factor>
         │        └─ <constant> ("pi")
         └─ ')'

```

---

## 🏗️ 5. Ce que tu dois implémenter

1. Une **structure `ExprNode` / `ASTNode`** représentant les opérations :

    ```rust
    enum ASTNode {
        Number(f64),
        Constant(String),
        UnaryOp { op: char, expr: Box<ASTNode> },
        BinaryOp { left: Box<ASTNode>, op: char, right: Box<ASTNode> },
        Function { name: String, arg: Box<ASTNode> },
    }
    ```

2. Un **parser récursif descendant** basé sur la grammaire :

    - `parse_expr()`
    - `parse_term()`
    - `parse_factor()`
    - `parse_primary()`

3. Gestion des erreurs claires :

    - Parenthèse fermante manquante
    - Fonction sans argument
    - Opérateur manquant entre deux valeurs

---

## ✅ Étapes à suivre maintenant

| Étape                  | À faire maintenant ? | Détail                                              |
| ---------------------- | -------------------- | --------------------------------------------------- |
| ✅ Tokenizer           | ✔️ Fait              | Tu as les bons tokens                               |
| 🛠️ Parser (`Expr`)     | 🔜 Prochaine étape   | Implémenter la grammaire récursive                  |
| 🔁 AST                 | 🔜 Intégré au parser | Construit à mesure qu’on parse                      |
| 📉 Évaluateur          | ⏳ Étape suivante    | Une fois l’AST construit, on l’évalue récursivement |
| ⚠️ Gestion des erreurs | Toujours             | À chaque étape : tokenizer, parser, évaluation      |
