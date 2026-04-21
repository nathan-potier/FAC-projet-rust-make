use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

const FICHIER_CACHE: &str = ".rustake_cache.json";

/// Structure qui représente notre fichier de sauvegarde
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cache {
    pub empreintes: HashMap<String, String>,
}

impl Cache {
    /// Charge le cache depuis le disque (ou crée un cache vide si introuvable)
    pub fn charger() -> Self {
        if let Ok(contenu) = fs::read_to_string(FICHIER_CACHE) {
            if let Ok(cache) = serde_json::from_str(&contenu) {
                return cache;
            }
        }
        Cache { empreintes: HashMap::new() }
    }

    /// Sauvegarde le cache sur le disque après un build
    pub fn sauvegarder(&self) {
        if let Ok(contenu) = serde_json::to_string_pretty(self) {
            let _ = fs::write(FICHIER_CACHE, contenu);
        }
    }
}

/// Calcule le hash Blake3 d'un fichier donné
pub fn calculer_hash_fichier(chemin: &str) -> Option<String> {
    // On lit les octets bruts du fichier
    if let Ok(contenu_fichier) = fs::read(chemin) {
        // On calcule l'empreinte avec Blake3
        let hash = blake3::hash(&contenu_fichier);
        // On convertit le hash en texte (hexadécimal)
        return Some(hash.to_hex().to_string());
    }
    None // Le fichier n'existe pas ou ne peut pas être lu
}