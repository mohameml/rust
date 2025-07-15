# Cour : **lifetime:**

## 1. **Introduction :**

-   **Définition:**

    > En Rust, un **lifetime** (durée de vie) est une manière de **modéliser le temps pendant lequel une référence reste valide**.

    -   Rust utilise les _lifetimes_ pour s'assurer qu’aucune **référence non valide (dangling reference)** ne soit utilisée — c’est-à-dire une référence vers une donnée qui n’existe plus ou a été libérée de la mémoire.

    -   En d'autres termes : Un _lifetime_ indique **jusqu’à quand une référence peut être utilisée en toute sécurité** sans qu’elle pointe vers une zone mémoire libérée ou modifiée de manière invalide.

-   Pourquoi les lifetimes sont-ils importants en Rust ?

    -   Rust garantit la **sécurité mémoire sans garbage collector**. Pour cela, il utilise un **système de possession (ownership)** et de **vérification d'emprunts (borrow checker)**.

    -   Les _lifetimes_ sont essentiels car :

        -   Ils **empêchent l’accès à des références invalides**.
        -   Ils **garantissent que les références vivent assez longtemps** pour être utilisées en toute sécurité.
        -   Ils permettent de **raisonner statiquement sur la durée de vie des données**, à la compilation.

    -   Autrement dit, grâce aux lifetimes, Rust **refuse de compiler un programme s’il détecte qu’une référence pourrait devenir invalide à l’exécution**.

-   **borrow checker**

    -   Le **borrow checker** est un composant du compilateur Rust chargé de vérifier que les règles d’ownership et de borrow sont respectées. Il :

        -   Suit les **emprunts (borrows)** dans le code.
        -   Vérifie qu’**aucune référence mutable ne coexiste avec une référence immuable**.
        -   Vérifie que **les références ne survivent pas à la donnée à laquelle elles pointent**.

    -   Pour faire cela, il utilise des **annotations de lifetime** explicites ou implicites.

-   **Exemple : Dangling Reference**

    Regardons un exemple simple où une référence devient invalide, c’est-à-dire un **dangling reference** :

    ```rust
    fn main() {
        let r;

        {
            let x = 5;
            r = &x; // ❌ Erreur ! `x` ne vit pas assez longtemps
        }

        println!("r: {}", r); // 🧨 Ici, `x` est déjà supprimé
    }
    ```

    -   La variable `x` est créée dans un bloc interne `{}`.
    -   On crée une **référence `r` vers `x`**, mais `x` **n’existe plus** en dehors du bloc !
    -   Résultat : `r` devient une **référence pendante (dangling)**.
    -   ✅ Le **borrow checker détecte cela à la compilation** et affiche une erreur comme :

> 💡 **Remarque importante :**
> Dans la **grande majorité des cas**, le **borrow checker est capable d’inférer automatiquement** la durée de vie des références.
> Cela signifie que **vous n’avez pas besoin de spécifier manuellement les lifetimes** dans votre code.
> Cependant, dans certains cas plus complexes (par exemple dans les **fonctions retournant des références**), Rust ne peut pas deviner le lifetime tout seul.
> Il faudra alors utiliser des **annotations explicites** appelées **"generic lifetimes"**, que nous aborderons dans les sections suivantes.

## 2. **generic lifetime annotations :**

-   **Définition:**

    -   En Rust, les _lifetimes_ (durées de vie) permettent au compilateur de s'assurer qu'aucune **référence invalide (dangling reference)** n'est utilisée.

    -   Par défaut, Rust peut inférer les _lifetimes_ dans la plupart des cas simples, mais dans des situations plus complexes (comme lorsqu’une fonction retourne une référence à l’un de ses paramètres), il faut **annoter explicitement les _lifetimes_**.

    > Une **generic lifetime annotation** est un nom abstrait (souvent `'a`) qui représente la durée de vie de certaines références dans une signature. Elle ne définit pas combien de temps une référence vit, mais **exprime une relation entre plusieurs références** (ex. : "la référence retournée vit au moins aussi longtemps que telle autre").

-   **Syntaxe:**

    La syntaxe générale d’une annotation de _lifetime_ dans une fonction est la suivante :

    ```rust
    fn nom_fonction<'a>(param1: &'a T) -> &'a T
    ```

    -   `<'a>` : indique qu’on introduit un _lifetime_ générique appelé `'a`
    -   `param1 &'a T` : une référence à un objet de type `T` qui vit au moins aussi longtemps que `'a`
    -   `-> &'a T` : la fonction retourne une référence qui a le même _lifetime_ `'a`.

-   **Exemple simple:**

    ```rust
    fn plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

    -   La fonction reçoit deux références à des chaînes (`x` et `y`) avec une durée de vie >= `'a`.
    -   Elle retourne une référence qui doit vivre **au moins aussi longtemps que** `'a`.
    -   Cela garantit que la référence retournée ne sera pas utilisée après que `x` ou `y` ait été libérée.

-   **Exemple 2:**

    ```rs

    fn main() {
        let s1 = String::from("longue chaîne");
        let result: &str;
        {
            let s2 = String::from("xyz");
            result = longest(s1.as_str(), s2.as_str());
            println!("La plus longue est {}", result); // OK
        }
        println!("La plus longue est {}", result); // ERREUR! s2 n'existe plus
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    ```

-   **Exemple 3:**

    ```rs
    #[derive(Debug)]
    struct Article<'a> {
        txt: &'a str,
    }

    fn main() {

        let a;
        {
            let txt: &str = "string info";

            a = Article { txt: txt };
        }

        println!("Article a = {:?}", a);

        println!("txt = {}", a.txt);
    }


    ```

## 3. **Cas d’usage des _lifetimes_ explicites en Rust:**

### 3.1. **Fonctions:**

-   **Condition :**

    > Vous devez spécifier des _lifetimes_ dans une fonction **uniquement si elle retourne une référence** à un ou plusieurs de ses paramètres empruntés. Rust ne peut pas toujours inférer automatiquement à quel paramètre appartient la référence de retour.

-   **Syntaxe :**

    ```rust
    fn nom_fonction<'a>(param1: &'a T, param2: &'a T) -> &'a T
    ```

-   **Exemple :**

    ```rust
    fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }
    ```

### 3.2 **Structures (`struct`):**

-   **Condition :**

    > Une structure nécessite des _lifetimes_ explicites **si elle contient au moins un champ qui est une référence**. Le compilateur doit garantir que les données référencées vivent au moins aussi longtemps que l'instance de la structure.

-   **Syntaxe :**

    ```rust
    struct NomStruct<'a> {
        champ: &'a T,
    }
    ```

-   **Exemple :**

    ```rust
    struct StrHolder<'a> {
        name: &'a str,
    }
    ```

### 3.3. **Blocs `impl`:**

-   **Condition :**
    Si une `struct` utilise des _lifetimes_, tout bloc `impl` associé doit aussi **inclure ces mêmes paramètres de _lifetime_** pour que les méthodes puissent les utiliser.

-   **Syntaxe :**

    ```rust
    impl<'a> Struct<'a> {
        fn methode(&self) -> &'a str {
            self.champ
        }
    }
    ```

-   **Exemple :**

    ```rust
    struct StrHolder<'a> {
        name: &'a str,
    }

    impl<'a> StrHolder<'a> {
        fn get_name(&self) -> &'a str {
            self.name
        }
    }
    ```

### 3.4. **Méthodes:**

-   **Condition :**

    > Si une méthode d’un `impl` ou d’un `trait` retourne une référence dépendante d’un argument, ou du `self`, vous devez annoter explicitement le _lifetime_ de retour.

-   **Cas 1 – Référence à un paramètre :**

    ```rust
    fn select<'a>(&self, s: &'a str) -> &'a str {
        s
    }
    ```

-   **Cas 2 – Référence à self :**

    ```rust
    fn get_name(&'a self) -> &'a str {
        self.name
    }
    ```

-   **Note :**

    > Les règles d’inférence de _lifetimes implicites_ dans les méthodes permettent de ne **pas** annoter explicitement dans certains cas simples, par exemple :

    ```rust
    fn get_name(&self) -> &str {
        self.name
    }
    ```

    > Rust applique des règles par défaut si le nombre de références est clair et unique.

### 3.5. **Traits:**

-   **Condition :**

    > Vous devez annoter les _lifetimes_ **dans la signature des méthodes du trait** **si** celles-ci manipulent ou retournent des références.

    > Si le trait dépend d’un _lifetime_ global, vous pouvez aussi déclarer un _lifetime_ pour le trait.

-   **Syntaxe :**

    ```rust
    trait Max {
        fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str;
    }
    ```

    ```rust
    trait Descriptible<'a> {
        fn description(&self) -> &'a str;
    }
    ```

### 3.6. **Enumérations (`enum`):**

-   **Condition :**

    > Vous devez annoter les _lifetimes_ **si une variante contient une référence**.

-   **Syntaxe :**

    ```rust
    enum Message<'a> {
        Borrowed(&'a str),
        Owned(String),
    }
    ```

-   **Exemple :**

    ```rust
    enum Either<'a> {
        First(&'a str),
        Second(i32),
    }
    ```

## 4. **lifetime elision rules:**

> Rust applique trois règles implicites pour **déduire automatiquement les _lifetimes_** dans la plupart des fonctions. Comprendre ces règles t’évitera d’avoir à écrire manuellement `'a` dans des cas simples, et t’aidera à savoir quand tu dois les annoter.

<!-- -   **Régles:**
    -   **Chaque paramètre de référence reçoit son propre _lifetime_ :**
        > Si une fonction a des paramètres de type `&T`, Rust assigne **un _lifetime_ distinct** à chacun. Cela permet au compilateur de raisonner indépendamment sur chacun.

    -  **S’il y a exactement un paramètre de référence, son _lifetime_ est affecté à la sortie**
 -->

### 4.1 Règle 1 — **Chaque paramètre de référence reçoit son propre _lifetime_**

-   **Définition :**

    > Si une fonction a des paramètres de type `&T`, Rust assigne **un _lifetime_ distinct** à chacun. Cela permet au compilateur de raisonner indépendamment sur chacun.

-   **Exemple 1:**

    ```rust
    fn compare(x: &str, y: &str) -> bool {
        x.len() > y.len()
    }
    ```

    ➡️ Ici, `x` et `y` reçoivent chacun un _lifetime_ différent implicitement (`x: &'a str`, `y: &'b str`). La fonction ne retourne aucune référence, donc pas besoin d'en propager.

-   **Exemple invalide (viol de la règle) :**

    ```rust
    fn compare_bad(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

    > Erreur : le compilateur ne sait pas **quel _lifetime_ associer à la sortie**. Ici, `x` et `y` ont deux _lifetimes_ différents, donc Rust ne peut pas en propager un seul sans annotation.

    > **Fix** :

    ```rust
    fn compare_fixed<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

### 4.2 Règle 2 — **S’il y a exactement un paramètre de référence, son _lifetime_ est affecté à la sortie**

-   **Définition :**

    > Si une fonction a **un seul paramètre de référence**, et retourne une référence, Rust **attribue automatiquement** le _lifetime_ du paramètre à la sortie.

-   **Exemple :**

    ```rust
    fn premiere_lettre(s: &str) -> &str {
        &s[..1]
    }
    ```

    ➡️ Le seul paramètre est `s`, donc Rust en déduit : `fn<'a>(s: &'a str) -> &'a str`.

-   **Exemple invalide :**

    ```rust
    fn premiere_lettre_mauvaise(x: &str, y: &str) -> &str {
        &x[..1]
    }
    ```

    > Il y a **deux références en entrée**, donc la règle 2 ne s’applique plus. Rust refuse car il ne peut pas deviner le _lifetime_ à propager à la sortie.

    > **Fix** :

    ```rust
    fn premiere_lettre_ok<'a>(x: &'a str, _: &str) -> &'a str {
        &x[..1]
    }
    ```

### 4.3 Règle 3 — **S’il y a plusieurs _lifetimes_ en entrée, mais l’un des paramètres est `&self` ou `&mut self`, la sortie hérite du _lifetime_ de `self`**

-   **Définition :**

    > Cette règle permet d’écrire des **méthodes d’objets** sans devoir expliciter les _lifetimes_ quand la sortie dépend de `self`.

-   **Exemple correct :**

    ```rust
    impl Texte {
        fn debut(&self) -> &str {
            &self.data[..1]
        }
    }
    ```

## 5. **Le _lifetime_ spécial : `'static`**

> Le _lifetime_ `'static` est un cas **particulier et important** dans Rust. Il désigne des données qui vivent **pendant toute la durée de vie du programme**.

-   **Définition**

    Un _lifetime_ `'static` signifie que la **référence est valable jusqu’à la fin du programme**. Cela peut correspondre à :

    -   des **littéraux statiques** (ex : `"hello"`),
    -   des **variables globales** statiques,
    -   ou des **données stockées dans le heap** et jamais libérées (ex : `Box::leak`).

-   **Exemple : littéraux string**

    ```rust
    fn hello() -> &'static str {
        "Hello, world!"  // type : &'static str
    }
    ```

    > Les littéraux string sont intégrés au binaire du programme. Ils vivent pendant toute l’exécution.

-   **Exemple 2: Mauvais usage de `'static`**

    ```rust
    fn faux<'a>(s: &'a str) -> &'static str {
        s  // ❌ Error : tu essaies d'étendre un lifetime local en static
    }
    ```

    > ❌ **Erreur de compilation** : la variable `s` n’est **pas garantie** de vivre aussi longtemps que `'static`. On ne peut pas _élargir_ un lifetime comme ça.

-   **Exemple 3 : `static` dans des constantes globales**

    ```rust
    static MESSAGE: &str = "Bienvenue !";

    fn lire() -> &'static str {
        MESSAGE
    }
    ```
