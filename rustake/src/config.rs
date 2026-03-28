use serde::Deserialize;
use std::collections::HashMap;

/// Représente le fichier Rustakefile.toml complet
#[derive(Debug, Deserialize)]
pub struct Rustakefile {
    pub targets: HashMap<String, Target>,
}

/// Représente une cible individuelle (ex: [targets.program])
#[derive(Debug, Deserialize)]
pub struct Target {
    // Si une cible n'a pas de dépendances, on utilisera un Vec vide par défaut
    #[serde(default)]
    pub depends: Vec<String>,
    // Une cible n'a pas forcément de commande donc option
    pub command: Option<String>,
}