mod cli;
mod config;

use clap::Parser;
use std::fs;
use std::process;
use std::collections::HashSet;

use cli::{Cli, Commands};
use config::Rustakefile;

use crate::config::Target;

fn main() {
    // Parsing des arguments de la ligne de commande
    let cli = Cli::parse();

    // Traitement des sous-commandes spéciales (ex: rustake init)
    if let Some(command) = cli.command {
        match command {
            Commands::Init => {
                creer_fichier_init(&cli.file);
                return;
            }
            Commands::Clean => {
                println!("La commande 'clean' n'est pas encore implémentée.");
                // TODO: Supprimer le cache et les cibles
                return;
            }
        }
    }

    // Lecture du fichier de configuration pour le build classique
    let config_content = match fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erreur : Impossible de lire le fichier '{}'.", cli.file);
            eprintln!("Raison : {}", e);
            eprintln!("Astuce : Avez-vous lancé 'rustake init' ?");
            process::exit(1);
        }
    };

    // Désérialisation du TOML vers nos struct
    let rustake_config: Rustakefile = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Erreur de syntaxe dans le fichier TOML : {}", e);
            process::exit(1);
        }
    };

    // Pour l'instant, on se contente d'afficher que tout a bien été lu
    println!("Fichier de configuration lu avec succès !");
    println!("Cibles disponibles : {:?}", rustake_config.targets.keys());

    // Identifier la cible à construire (par défaut ou demandée par l'utilisateur)
    let cible_demandee = cli.target.unwrap_or_else(|| String::from("default"));
    println!("L'utilisateur veut construire : {}", cible_demandee);

    // Resolution du dag
    let mut ordre_execution = Vec::new();
    let mut visitees = HashSet::new();
    let mut en_cours_de_visite = HashSet::new();

    match resoudre_dependances(
        &cible_demandee,
        &rustake_config.targets,
        &mut ordre_execution,
        &mut visitees,
        &mut en_cours_de_visite,
    ) {
        Ok(_) => {
            println!("Ordre de construction déterminé avec succès :");
            for etape in &ordre_execution {
                println!(" -> {}", etape);
            }
        }
        Err(erreur) => {
            eprintln!("Erreur : {}", erreur);
            process::exit(1);
        }
    }
    // TODO: Vérification des hashs (blake3)
    // TODO: Exécution des commandes (std::process::Command)
}

/// Fonction utilitaire pour la commande `rustake init`
fn creer_fichier_init(nom_fichier: &str) {
    let contenu_par_defaut = r#"
    [targets.program]
    depends = ["src/main.rs", "src/lib.rs"]
    command = "rustc src/main.rs -o program"

    [targets.test]
    depends = ["src/lib.rs", "tests/test.rs"]
    command = "cargo test"

    [targets.default]
    depends = ["program"]
    "#;

    if std::path::Path::new(nom_fichier).exists() {
        eprintln!("Erreur : Le fichier '{}' existe déjà.", nom_fichier);
        process::exit(1);
    }

    match fs::write(nom_fichier, contenu_par_defaut.trim()) {
        Ok(_) => println!("Projet initialisé avec succès (création de '{}').", nom_fichier),
        Err(e) => eprintln!("Erreur lors de la création du fichier : {}", e),
    }
}

/// Résout l'ordre des dépendances en évitant les cycles
fn resoudre_dependances<'a>(
    nom_cible: &'a str,
    toutes_les_cibles: &'a std::collections::HashMap<String, Target>,
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