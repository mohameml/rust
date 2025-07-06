## 🎯 Objectif

Écrire un **parser récursif descendant** qui, à partir de ta grammaire LL(1), transforme un `Vec<Token>` en `Expr`.

---

## 🧱 Étapes recommandées

### 1. 📦 Création de la structure `Parser`

```rust
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
```

Et une méthode pour accéder au token courant :

```rust
impl Parser {
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
```

---

### 2. 🧠 Implémenter les fonctions récursives

Tu vas suivre la grammaire suivante pour ton parser :

```bnf
Expr        → Term (("+" | "-") Term)*
Term        → Factor (("*" | "/") Factor)*
Factor      → ("+" | "-")? Primary
Primary     → number | const | Function "(" Expr ")" | "(" Expr ")"
```

Et tu traduis ça en :

```rust
impl Parser {
    fn parse_expr(&mut self) -> Option<Expr> { ... }

    fn parse_term(&mut self) -> Option<Term> { ... }

    fn parse_factor(&mut self) -> Option<Factor> { ... }

    fn parse_primary(&mut self) -> Option<Primary> { ... }
}
```

---

### 3. 💡 Exemple de `parse_primary`

```rust
fn parse_primary(&mut self) -> Option<Primary> {
    match self.current()? {
        Token::Float(f) => {
            self.advance();
            Some(Primary::Number(*f))
        }
        Token::Int(i) => {
            self.advance();
            Some(Primary::Number(*i as f64))
        }
        Token::Constant(cst) => {
            self.advance();
            let constant = match cst.as_str() {
                "pi" => Constant::Pi,
                "e" => Constant::E,
                _ => return None,
            };
            Some(Primary::Const(constant))
        }
        Token::Function(name) => {
            let function = match name.as_str() {
                "cos" => Function::Cos,
                "sin" => Function::Sin,
                "tan" => Function::Tan,
                "ln" => Function::Ln,
                _ => return None,
            };
            self.advance(); // consume function
            self.expect(Token::LParen)?;
            self.advance(); // consume '('
            let expr = self.parse_expr()?;
            self.expect(Token::RParen)?;
            self.advance(); // consume ')'
            Some(Primary::FunctionCall(function, Box::new(expr)))
        }
        Token::LParen => {
            self.advance();
            let expr = self.parse_expr()?;
            self.expect(Token::RParen)?;
            self.advance();
            Some(Primary::GroupedExpr(Box::new(expr)))
        }
        _ => None,
    }
}
```

---

### 4. 🔁 Gestion des `Add`, `Sub`, `Mul`, `Div`

Dans `parse_expr` :

```rust
fn parse_expr(&mut self) -> Option<Expr> {
    let mut node = Expr::Term(Box::new(self.parse_term()?));
    while let Some(Token::Operator(op)) = self.current() {
        match op {
            Operator::Plus | Operator::Sub => {
                let op_token = op.clone();
                self.advance();
                let right = self.parse_term()?;
                node = match op_token {
                    Operator::Plus => Expr::Add(Box::new(node), Box::new(right)),
                    Operator::Sub => Expr::Sub(Box::new(node), Box::new(right)),
                    _ => unreachable!(),
                };
            }
            _ => break,
        }
    }
    Some(node)
}
```

Et même logique pour `parse_term`.

---

### 5. ✅ Fonction `expect`

Pour faciliter les contrôles :

```rust
fn expect(&self, token: Token) -> Option<()> {
    if let Some(current) = self.current() {
        if *current == token {
            return Some(());
        }
    }
    None
}
```

---

## ✅ Conclusion

Une fois tout cela mis en place, tu pourras faire :

```rust
let mut parser = Parser { tokens, pos: 0 };
let expr = parser.parse_expr().expect("Erreur de parsing");
pretty_print(&expr, 0);
```
