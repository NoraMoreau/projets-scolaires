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
    #[arg(long = "lexicographic-sort")]
    #[arg(conflicts_with = "filter")]
    lexico: bool,
    #[clap(long = "filter")]
    #[arg(conflicts_with = "lexico")]
    filter: Option<String>,
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

//écrire en ldc 
//cargo run usage "src/test/sous_dossier"
//cargo run -- --lexicographic-sort usage "src/test/sous_dossier"
//cargo run -- --filter mot usage "src/test/sous_dossier" 
fn main() -> std::io::Result<()> {
    //Crée une instance de Cli
    let cli = Cli::parse();

    //Vérifie le pattern de cli.command pour savoir comment réagir en fonction de la commande passée
    match &cli.command {
        Commands::Usage {path} => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            let tree = FileTree::new(path);
            if cli.lexico == true {
                tree?.show(true, None);
            } else if let Some(filter) = cli.filter {
                tree?.show(false, Some(filter));
            } else {
                tree?.show(false, None);
            }
        }
    }
    Ok(())
}
