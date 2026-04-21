use std::collections::{HashMap, HashSet};
use crate::config::Target;

/// Résout l'ordre des dépendances en évitant les cycles
pub fn resoudre_dependances<'a>(
    nom_cible: &'a str,
    toutes_les_cibles: &'a HashMap<String, Target>,
    ordre_execution: &mut Vec<&'a str>,
    visitees: &mut HashSet<&'a str>,
    en_cours_de_visite: &mut HashSet<&'a str>,
) -> Result<(), String> {
    
    // Détection de dépendance circulaire
    if en_cours_de_visite.contains(nom_cible) {
        return Err(format!("Dépendance circulaire détectée sur '{}' !", nom_cible));
    }

    // Si la cible a déjà été traitée, on l'ignore
    if visitees.contains(nom_cible) {
        return Ok(());
    }

    // On marque la cible comme étant en cours d'analyse
    en_cours_de_visite.insert(nom_cible);

    // On cherche si le nom correspond à une cible du TOML
    if let Some(cible) = toutes_les_cibles.get(nom_cible) {
        // On parcourt la liste des dépendances
        for dependance in cible.depends.iter() {
            resoudre_dependances(
                dependance,
                toutes_les_cibles,
                ordre_execution,
                visitees,
                en_cours_de_visite,
            )?; 
        }
    }

    // Fin de l'analyse pour cette cible
    en_cours_de_visite.remove(nom_cible);
    visitees.insert(nom_cible);

    // On l'ajoute à l'ordre d'exécution
    ordre_execution.push(nom_cible);

    Ok(())
}