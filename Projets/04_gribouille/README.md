Parfait. Voici une **version détaillée de l’énoncé du projet** adapté au **langage Rust**, en gardant l’esprit du projet C original :

---

## 🦀 Projet Rust – Manipulation d’images PPM avec motifs, tracés et remplissage

### 🎯 **Objectif général :**

Développer une application en Rust capable de manipuler des images au format **PPM (Portable PixMap)** en utilisant des opérations basiques comme le tracé de formes, le remplissage de zones, l’application de motifs, et l’exportation d’image.

---

## 📦 **Contraintes et exigences techniques :**

### 🧱 Structure modulaire :

-   Le projet doit être découpé en **modules Rust** clairs (`mod.rs`, etc.) :

    -   Gestion du canevas/image.
    -   Représentation des pixels.
    -   Algorithmes de tracé.
    -   Algorithme de remplissage (flood fill).
    -   Bibliothèque de motifs.
    -   Exportation vers le format `.ppm`.

---

## 🖼️ **Partie 1 — Canevas et structure `Image`**

-   Implémente une structure `Pixel` contenant 3 composantes : rouge, vert, bleu (valeurs entières entre 0 et 255).
-   Implémente une structure `Image` représentant une image RGB sous forme d’un tableau de pixels.
-   Le canevas doit être initialisé avec une couleur par défaut (ex. blanc).
-   Fournir des méthodes pour :

    -   Accéder à un pixel (en lecture et écriture).
    -   Modifier un pixel.
    -   Exporter l’image au format **PPM P3**.

---

## 🧱 **Partie 2 — Tracés simples**

-   Implémente des fonctions pour tracer des **formes géométriques simples** :

    -   Segments horizontaux, verticaux, diagonaux.
    -   Rectangle vide ou plein.
    -   Cercle plein ou vide (optionnel).

-   Le tracé se fera sur le canevas, en utilisant une couleur donnée.

---

## 🎨 **Partie 3 — Remplissage (Flood Fill)**

-   Implémente une fonction de **remplissage par diffusion (flood fill)** :

    -   À partir d’un point donné, remplit la zone avec une nouvelle couleur.
    -   Le remplissage doit s’arrêter lorsque la couleur rencontrée n’est plus la couleur initiale.
    -   Deux versions possibles : récursive ou itérative (pile, file…).

---

## 🧩 **Partie 4 — Motifs**

-   Implémente une **bibliothèque de motifs** (motifs de petite taille, ex. 4x4).
-   Chaque motif est une mini-image (structure `Pattern`) que l’on peut :

    -   Appliquer à une position donnée dans une image.
    -   Répéter plusieurs fois à la suite dans l’image (tiling).

-   Le motif peut être défini :

    -   Par défaut dans le code (statiquement).
    -   Ou bien chargé depuis un fichier texte ou JSON (bonus).

---

## 💾 **Partie 5 — Export de l’image**

-   L’image finale doit être exportée au format **PPM ASCII (type P3)** :

    -   Avec les informations de taille, codage RGB.
    -   Le fichier doit être lisible par un visualiseur d’image compatible PPM (ex. GIMP, ImageMagick).

---

## 🔧 **Fonctionnalités bonus possibles (facultatives)**

-   Ajout de motifs dynamiques définis par l'utilisateur.
-   Rotation de motif (à 90°, 180°, 270°).
-   Création d’une petite interface CLI pour appliquer des commandes sur l’image :

    -   `draw-rect`, `fill`, `apply-pattern`, `export`, etc.

-   Gestion d’arguments via `clap` ou `structopt`.
-   Tests unitaires avec `#[test]`.

---

## ✍️ **Livrables attendus**

-   Code source Rust complet avec fichiers bien structurés.
-   Un script `main.rs` permettant de démontrer les différentes fonctionnalités.
-   Quelques fichiers `.ppm` générés en sortie.
-   Documentation minimale (README) expliquant comment compiler, exécuter et tester le projet.

---

Souhaites-tu que je rédige une version PDF propre de cet énoncé ?

# Help :

Voici comment tu pourrais structurer et implémenter ton projet en Rust, en gardant une architecture claire, modulaire et idiomatique.

---

## 🧱 **Structure du projet**

```
image_project/
│
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs               // module principal
    ├── image/
    │   ├── mod.rs           // gestion du canevas/image
    │   ├── pixel.rs         // struct Pixel
    ├── fill.rs              // algorithme de remplissage
    ├── pattern/
    │   ├── mod.rs           // gestion des motifs
    │   ├── pattern.rs       // définition des motifs
    └── export.rs            // export PPM
```

---

## 🔧 **1. Struct `Pixel`** (dans `image/pixel.rs`)

```rust
#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
}
```

---

## 🖼️ **2. Struct `Image` (canvas)** (`image/mod.rs`)

```rust
use crate::image::pixel::Pixel;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize, bg: Pixel) -> Self {
        Self {
            width,
            height,
            pixels: vec![bg; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Pixel> {
        if x < self.width && y < self.height {
            Some(self.pixels[y * self.width + x])
        } else {
            None
        }
    }
}
```

---

## 🧩 **3. Les motifs** (`pattern/pattern.rs`)

```rust
use crate::image::pixel::Pixel;
use crate::image::Image;

pub struct Pattern {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Pixel>,
}

impl Pattern {
    pub fn new(width: usize, height: usize, data: Vec<Pixel>) -> Self {
        assert_eq!(width * height, data.len());
        Self { width, height, data }
    }

    pub fn apply(&self, image: &mut Image, x0: usize, y0: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(color) = self.data.get(y * self.width + x) {
                    image.set_pixel(x0 + x, y0 + y, *color);
                }
            }
        }
    }
}
```

---

## 🌊 **4. Algorithme de remplissage** (`fill.rs`)

Flood fill récursif ou itératif :

```rust
use crate::image::{Image, pixel::Pixel};
use std::collections::VecDeque;

pub fn flood_fill(image: &mut Image, x: usize, y: usize, target: Pixel, replacement: Pixel) {
    if target == replacement {
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        if let Some(pixel) = image.get_pixel(cx, cy) {
            if pixel == target {
                image.set_pixel(cx, cy, replacement);
                if cx > 0 {
                    queue.push_back((cx - 1, cy));
                }
                if cy > 0 {
                    queue.push_back((cx, cy - 1));
                }
                if cx + 1 < image.width {
                    queue.push_back((cx + 1, cy));
                }
                if cy + 1 < image.height {
                    queue.push_back((cx, cy + 1));
                }
            }
        }
    }
}
```

---

## 🖨️ **5. Export en PPM** (`export.rs`)

```rust
use crate::image::Image;
use std::fs::File;
use std::io::{Result, Write};

pub fn export_to_ppm(image: &Image, filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "P3\n{} {}\n255", image.width, image.height)?;

    for pixel in &image.pixels {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
    }

    Ok(())
}
```

---

## 🧪 **6. Main de test** (`main.rs`)

```rust
mod image;
mod fill;
mod export;
mod pattern;

use image::{Image, pixel::Pixel};
use pattern::pattern::Pattern;
use export::export_to_ppm;

fn main() {
    let mut img = Image::new(100, 100, Pixel::white());

    // Exemple de motif 2x2 noir
    let pattern = Pattern::new(2, 2, vec![Pixel::black(); 4]);
    pattern.apply(&mut img, 10, 10);

    // Remplissage à partir de (0, 0) avec gris
    fill::flood_fill(&mut img, 0, 0, Pixel::white(), Pixel::new(200, 200, 200));

    export_to_ppm(&img, "output.ppm").expect("Failed to save image");
}
```

---

## ✅ **Avantages de cette structure :**

-   **Lisibilité** : séparation claire entre image, motif, remplissage, export.
-   **Extensibilité** : tu peux facilement ajouter d'autres algos (ex: rotation de motif, transformation affine).
-   **Rust idiomatique** : usage de `Option`, modules clairs, pas de `unsafe`.

---

Souhaites-tu que je t’aide à convertir précisément un fichier `.c` que tu possèdes ? Ou à ajouter des opérations plus complexes comme des rotations, zoom, interpolation, etc. ?
