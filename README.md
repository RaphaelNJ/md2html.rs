# Md2Html.rs

Ce programme est un convertisseur Markdown vers HTML développé en Rust. Il utilise, en interne des expressions régulières (RegEx). Le convertisseur est équipé d'un petit serveur et d'un "watcher", un composant de l'API Linux qui permet d'être notifié lors de la modification d'un fichier. Cela lui permet donc d'actualiser le fichier HTML dès que le Markdown est modifié, offrant ainsi un aperçu en temps réel du rendu HTML.

Veuillez noter que ce programme a été exclusivement testé sur un environnement Linux. Sa compatibilité avec Windows ou MacOS n'a pas été vérifiée.

## Fonctionnalités

- Convertit des fichiers Markdown au format HTML
- Utilise des expressions régulières pour la conversion
- Fournit un aperçu en temps réel du rendu HTML grâce au serveur et au "watcher"

## Limitations et choix de conception

Il est important de souligner que la technique utilisée dans ce programme, qui repose sur des Regex pour effectuer le parsing du Markdown en HTML, est considérée comme une approche relativement limitée. En général, il est recommandé d'utiliser un arbre syntaxique basé sur un analyseur lexical (lexer) pour analyser le Markdown et ensuite le convertir en HTML. Cela permet une meilleure gestion des structures syntaxiques complexes et offre plus de flexibilité.

Cependant, ce programme a été conçu dans un souci de simplicité, en se concentrant sur les cas les plus courants du Markdown. Bien que l'utilisation d'expressions régulières puisse présenter certaines limites dans des situations plus complexes, elle reste pertinente dans le cadre de ce programme.

Si vous avez des besoins spécifiques ou souhaitez prendre en charge des fonctionnalités Markdown plus avancées, il est recommandé d'envisager des bibliothèques dédiées ou des approches basées sur un arbre syntaxique.

## Démo

https://github.com/RaphaelNJ/md2html.rs/assets/52333330/31f13841-4e6f-4e0e-9f2a-62448dd966fc

## Prérequis

- Langage de programmation Rust
- Système d'exploitation Linux (uniquement testé sous Linux)

## Installation

1. Clonez le dépôt :

```shell
git clone https://github.com/RaphaelNJ/md2html.rs.git
```

2. Accédez au répertoire du projet :

```shell
cd md2html.rs
```

3. Compilez le projet avec Cargo :

```shell
cargo build --release
```

## Utilisation

1. Exécutez la commande suivante pour lancer le programme :

```shell
target/release/md_rs <répertoire_markdown> <répertoire_output>
```

Assurez-vous de remplacer `<répertoire_markdown>` par le chemin du répertoire contenant les fichiers Markdown à convertir, et `<répertoire_output>` par le chemin du répertoire où vous souhaitez enregistrer les fichiers HTML générés.

2. Le programme convertira automatiquement tous les fichiers Markdown du répertoire spécifié en fichiers HTML et les enregistrera dans le répertoire de sortie.

3. Si vous apportez des modifications aux fichiers Markdown, le programme les détectera et mettra à jour les fichiers HTML correspondants.

## Licence

Ce programme est distribué sous la [licence MIT](https://opensource.org/licenses/MIT).
