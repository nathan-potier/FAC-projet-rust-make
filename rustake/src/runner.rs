use std::process::Command;
use std::collections::HashMap;
use crate::config::Target;

/// Parcourt l'ordre d'exécution et lance les commandes système
pub fn executer_cibles(
    ordre_execution: &[&str],
    toutes_les_cibles: &HashMap<String, Target>,
) -> Result<(), String> {
    
    // On boucle sur chaque cible dans le bon ordre
    for &nom_cible in ordre_execution {
        
        // On récupère la recette (Target) de cette cible
        if let Some(cible) = toutes_les_cibles.get(nom_cible) {
            
            // On vérifie si la cible a une commande (Option<String>)
            if let Some(commande_str) = &cible.command {
                println!("Exécution de [{}] : {}", nom_cible, commande_str);

                // On découpe la phrase par les espaces 
                // "rustc main.rs" devient un itérateur sur "rustc" puis "main.rs"
                let mut parties = commande_str.split_whitespace();
                
                // Le tout premier mot est le nom du programme 
                if let Some(programme) = parties.next() {
                    
                    // Tout le reste, ce sont les arguments
                    let arguments: Vec<&str> = parties.collect();

                    // On lance la commande sur l'ordinateur
                    let statut = Command::new(programme)
                        .args(&arguments)
                        .status() // .status() attend que la commande se termine
                        .map_err(|e| format!("Impossible de lancer '{}' : {}", programme, e))?;

                    if !statut.success() {
                        return Err(format!(
                            "Échec de la cible '{}' ! La commande a renvoyé une erreur.", 
                            nom_cible
                        ));
                    }
                }
            } else {
                //cible de regroupement
                println!("Cible [{}] atteinte (aucune commande à exécuter).", nom_cible);
            }
        }
    }

    Ok(())
}