mod file_tree;
mod print_tree;
mod size;

//clap: bibliothèque utilisée pour analyser et valider la chaîne d'arguments de ligne 
//de commande fournie par l'utilisateur au moment de l'exécution
use clap::{Parser, Subcommand};
use file_tree::FileTree;
use std::path::{Path, PathBuf};

//compilateur génére automatiquement le code d'implémentation de nom du trait pour la structure
#[derive(Parser)] 
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the disk usage tree for the given path
    Usage {
        /// (default '.')
        path: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    //Crée une instance de Cli
    let cli = Cli::parse();

    //Vérifie que si cli.command a dans Usa
    match &cli.command {
        Commands::Usage { path } => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            FileTree::new(path)?.show();
        }
    }
    Ok(())
}
