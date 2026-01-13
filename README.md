![License](https://img.shields.io/badge/License-Bastien--mrq-orange)
![Version](https://img.shields.io/badge/Version-2.0.0-orange)
# img-to-ascii

🇫🇷 [Version française](#-version-française) | 🇬🇧 [English version](#-english-version)

---

## 🇫🇷 Version française
Un outil en ligne de commande Rust qui convertit les images en art ASCII avec plusieurs styles de sortie et des dimensions configurables.

### Fonctionalités

- Convertir n'importe quelle format d'image en ASCII
- Plusieurs style ASCII  (simple, detailed, block, intensity, minimalist, circle)
- Largeur de sortie configurable
- Sortie dans la console ou dans un fichier
- Maintient le ratio d'aspect avec un ajustement d'échelle des caractères.
- Sortie console en couleur
### Installation

#### Prérequis
- Rust (1.70 or later)
- Cargo

#### Construire depuis les fichiers sources
```bash
git clone https://github.com/bastien-mrq/img-to-ascii
cd img-to-ascii
cargo build --release
```

### Utilisatiion

#### Utilisation basique
```bash
cargo run -- input.jpg
```

#### Utilisation avancée
```bash
cargo run -- input.jpg --output=file --name=output.txt --style=detailed --width=120 --color=true
```

#### Argument en ligne de commande

- `input` - Chemin vers l'image d'entrée (requis)
- `--output` - Destination de sortie : `console` (défaut) or `file`
- `--name` - Nom du fichier de sortie quand la sortie est fichier (défaut : "name")
- `--style` - style d'ASCII art  (défaut: "default")
- `--width` - Largeur de sortie en caractère (défaut : 80)
- `--color`- Afficher la sortie en couleur (défaut : false)

#### Styles d'ASCII Art 

- **simple**: ` .-+*#@`
- **detailed**: `  .':-=+o*%#@$`
- **block**: `  ░░▒▒▓▓██`
- **intensity**: ` .,:;ox%#@`
- **minimalist**: ` .-=*#`
- **circle**: ` .oO0`
- **default**: ` .:-=+*#%@`

### Examples

#### Sortie console avec le style detailed 
```bash
cargo run -- photo.png --style detailed --width 100
```

#### Enregistrement fichier avec le style block
```bash
cargo run -- image.jpg --output file --name ascii_art.txt --style block --width 80
```

### dépendance

- `image` (0.24) - Traitement des images et support des fichiers
- `clap` (4.0) - Passer des arguments en ligne de commande
- `owo-colors` (4.2.3) - affichage des caractères en couleurs dans la console


### Comment ca marche

1. **Chargement d'image** : Utilisation du crate `image` Pour charger les différents format d'image
2. **Redimensionnement** : Redimensionne l'image a la largeur demandé en gardant le ratio d'aspect
3. **Conversion échelle de gris** : Convertis l'image en echelle de gris pour récupérer la luminance
4. **Conversion en ASCII** : Convertis la luminance en caractère ASCII correspondent au style sélectionné
5. **Couleur** : Convertis les pixels en couleur RVB
6. **Sortie** : Affiche la sortie dans la console ou enregistre dans le fichier

L'outil applique un coefficient d'ajustement de caractères (0,45) pour compenser le rapport hauteur-largeur typique des caractères de terminal, assurant ainsi que l'art ASCII conserve des proportions appropriées.
### Structure du projet

```
src/
├── main.rs                 # Interface CLI et Logique principale
├── lib.rs                  # Point d'entré des librairie
└── services/
    ├── mod.rs              # Exports des modules service
    └── img_to_ascii.rs     # Logique de convertion ASCII
```
---
## 🇬🇧 Version anglaise
A Rust command-line tool that converts images to ASCII art with multiple output styles and configurable dimensions.

### Features

- Convert any image format to ASCII art
- Multiple ASCII art styles (simple, detailed, block, intensity, minimalist, circle)
- Configurable output width
- Console or file output
- Maintains aspect ratio with character scaling adjustment
- Console output in color

### Installation

#### Prerequisites
- Rust (1.70 or later)
- Cargo

#### Build from source
```bash
git clone https://github.com/bastien-mrq/img-to-ascii
cd img-to-ascii
cargo build --release
```

### Usage

#### Basic usage
```bash
cargo run -- input.jpg
```

#### Advanced options
```bash
cargo run -- input.jpg --output=file --name=output.txt --style=detailed --width=120 --color=true
```

#### Command-line arguments

- `input` - Path to the input image file (required)
- `--output` - Output destination: `console` (default) or `file`
- `--name` - Output filename when using file output (default: "name")
- `--style` - ASCII art style (default: "style")
- `--width` - Output width in characters (default: 80)
- `--color`- Show the art in color (default: false)

#### ASCII Art Styles

- **simple**: ` .-+*#@`
- **detailed**: `  .':-=+o*%#@$`
- **block**: `  ░░▒▒▓▓██`
- **intensity**: ` .,:;ox%#@`
- **minimalist**: ` .-=*#`
- **circle**: ` .oO0`
- **default**: ` .:-=+*#%@`

### Examples

#### Console output with detailed style
```bash
cargo run -- photo.png --style detailed --width 100
```

#### Save to file with block style
```bash
cargo run -- image.jpg --output file --name ascii_art.txt --style block --width 80
```

### Dependencies

- `image` (0.24) - Image processing and format support
- `clap` (4.0) - Command-line argument parsing
- `owo-colors` (4.2.3) - Display chars in color in the console

### How it works

1. **Image Loading**: Uses the `image` crate to load various image formats
2. **Resizing**: Scales the image to the target width while maintaining aspect ratio
3. **Grayscale Conversion**: Converts the image to grayscale for brightness mapping
4. **ASCII Mapping**: Maps pixel brightness values to ASCII characters based on the selected style
5. **Color**: Maps pixels color
6. **Output**: Displays the result in console or saves to a file

The tool applies a character ratio adjustment (0.45) to account for the typical height-to-width ratio of terminal characters, ensuring the ASCII art maintains proper proportions.

### Project Structure

```
src/
├── main.rs                 # CLI interface and main logic
├── lib.rs                  # Library entry point
└── services/
    ├── mod.rs              # Services module exports
    └── img_to_ascii.rs     # Core ASCII conversion logic
```

## License



🇫🇷**Usage libre, revente interdite.**\
🇬🇧**Free usage, sell it forbiden.**

### Vous pouvez / You can:

✅ Utiliser ce logiciel pour tout / Use this software for anything \
✅ Le modifier comme vous voulez / Change it however you want \
✅ Le partager avec d'autres / Share it with others \
✅ L'utiliser dans votre entreprise / Use it in your business

### Vous ne pouvez pas / You cannot:
❌ Vendre ce logiciel / Sell this software \
❌ Prétendre que c'est vous qui l'avez créé / Claim you made it

Licence complète : [LICENSE](LICENSE)\
Complete licence : [LICENSE](LICENSE)
