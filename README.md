# rustake - Outil de build en Rust

## But du projet
`rustake` est un outil de build en ligne de commande inspiré de `make`. Son objectif est d'automatiser la compilation et la gestion des dépendances dans les projets logiciels en s'assurant que seules les cibles nécessaires sont reconstruites.

## Fonctionnalités
- **Initialisation** : Création d'un fichier de configuration par défaut via `rustake init`.
- **Résolution de dépendances** : Création d'un graphe d'exécution (DAG) avec détection des dépendances circulaires.
- **Cache intelligent** : Détection des changements de fichiers basée sur le hachage (Blake3) et non sur les timestamps.
- **Nettoyage** : Suppression des artefacts générés et du cache via `rustake clean`.

## Particularités techniques
- **Langage** : Écrit intégralement en Rust pour la sécurité et la performance.
- **Hachage** : Utilisation de la crate `blake3` pour générer des empreintes numériques des fichiers, ce qui est beaucoup plus fiable que les dates de modification (timestamps) traditionnellement utilisées par `make`.
- **Sérialisation** : Utilisation de `serde` et `toml` pour parser le fichier `Rustakefile.toml`.

---

## Utilisation et Démonstration
### 1. Environnement de base
Assurez-vous d'avoir un fichier source et un fichier de configuration prêts à l'emploi.
```bash
cat hello.rs
cat Rustakefile.toml