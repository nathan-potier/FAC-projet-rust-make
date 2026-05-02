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

// TESTS UNITAIRES

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependance_simple_sans_cycle() {
        let mut cibles = HashMap::new();
        cibles.insert("programme".to_string(),Target { depends: vec!["main.rs".to_string()], command: None });

        let mut ordre = Vec::new();
        let mut visitees = HashSet::new();
        let mut en_cours = HashSet::new();

        let resultat = resoudre_dependances("programme", &cibles, &mut ordre, &mut visitees, &mut en_cours);
        
        assert!(resultat.is_ok(), "Le graphe simple ne devrait pas renvoyer d'erreur");
        assert_eq!(ordre, vec!["main.rs", "programme"]);
    }

    #[test]
    fn test_dependance_circulaire_detectee() {
        let mut cibles = HashMap::new();
        // A dépend de B
        cibles.insert(
            "A".to_string(),
            Target { depends: vec!["B".to_string()], command: None }
        );
        // B dépend de A
        cibles.insert(
            "B".to_string(),
            Target { depends: vec!["A".to_string()], command: None }
        );

        let mut ordre = Vec::new();
        let mut visitees = HashSet::new();
        let mut en_cours = HashSet::new();

        let resultat = resoudre_dependances("A", &cibles, &mut ordre, &mut visitees, &mut en_cours);
        
        // On s'attend à ce que ça échoue (is_err)
        assert!(resultat.is_err(), "Le cycle A -> B -> A n'a pas été détecté !");
        
        // On vérifie même le message d'erreur
        if let Err(msg) = resultat {
            assert!(msg.contains("circulaire"), "Le message d'erreur ne parle pas de cycle circulaire");
        }
    }
}