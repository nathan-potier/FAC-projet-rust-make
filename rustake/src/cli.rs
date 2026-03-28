use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rustake")]
pub struct Cli {
    /// Fichier de configuration à utiliser
    #[arg(short, long, default_value = "Rustakefile.toml")]
    pub file: String,

    /// Les sous-commandes disponibles (init, clean)
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Cible spécifique à builder (si on tape juste "rustake program")
    pub target: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Crée un Rustakefile.toml par défaut
    Init,
    /// Supprime les cibles générées et le cache
    Clean,
}