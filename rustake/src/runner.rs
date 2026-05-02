use std::process::Command;
use std::collections::HashMap;
use crate::config::Target;
use crate::cache::{Cache, calculer_hash_fichier};

/// Parcourt l'ordre d'exécution et lance les commandes système
pub fn executer_cibles(
    ordre_execution: &[&str],
    toutes_les_cibles: &HashMap<String, Target>,
    cache: &mut Cache,
) -> Result<(), String> {
    
    // On boucle sur chaque cible dans le bon ordre
    for &nom_cible in ordre_execution {
        
        // On récupère la recette (Target) de cette cible
        if let Some(cible) = toutes_les_cibles.get(nom_cible) {
            
            let mut a_recompiler = false;
            let mut nouveaux_hashes = HashMap::new();

            // Vérifier si le fichier cible existe
            // On ne le fait que si la cible a une commande
            if cible.command.is_some() && !std::path::Path::new(nom_cible).exists() {
                println!("La cible [{}] n'existe pas sur le disque, on doit la créer.", nom_cible);
                a_recompiler = true;
            }

            // On vérifie les dépendances de cette cible
            for dep in &cible.depends {
                if let Some(nouveau_hash) = calculer_hash_fichier(dep) {
                    // On compare avec l'ancien hash connu
                    if cache.empreintes.get(dep) != Some(&nouveau_hash) {
                        a_recompiler = true;
                    }
                    nouveaux_hashes.insert(dep.clone(), nouveau_hash);
                } else {
                    // Si on ne peut pas hasher la dépendance (ex: c'est un nom de cible, pas un fichier)
                    a_recompiler = true; 
                }
            }

            // Si c'est juste une cible de regroupement (ex: "default")
            if cible.command.is_none() {
                continue;
            }

            // Si rien n'a changé, on saute l'exécution
            if !a_recompiler {
                println!("C'est bon, [{}] est déjà à jour.", nom_cible);
                continue;
            }

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
                        return Err(format!("Échec de la cible '{}'.", nom_cible));
                    }

                    // L'exécution a réussi : On met à jour le cache avec les nouveaux hashs
                    for (dep, hash) in nouveaux_hashes {
                        cache.empreintes.insert(dep, hash);
                    }
                }
            }
        }
    }

    Ok(())
}