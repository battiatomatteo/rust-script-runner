use colored::*;
use dialoguer::{Select, Input};

use crate::config::{Config, salva_config};
use crate::file_ops::{leggi_file, descrizione_file, esegui_file};

pub fn menu_principale() {
    let mut config = crate::config::carica_config();
    let mut path = config.cartella.clone();

    loop {
        if config.menu.colori {
            println!("\n{}", "=== MENU PRINCIPALE ===".bold().blue());
            println!("Cartella attuale: {}", path.yellow());
        } else {
            println!("\n=== MENU PRINCIPALE ===");
            println!("Cartella attuale: {}", path);
        }

        let opzioni = vec![
            "Lista script ed esegui",
            "Cambia cartella",
            "Esci",
        ];

        let scelta = Select::new()
            .with_prompt("Seleziona un'opzione")
            .items(&opzioni)
            .default(0)
            .interact()
            .unwrap();

        match scelta {
            0 => {
                let files = leggi_file(&path);
                if files.is_empty() {
                    println!("Nessun file trovato.");
                } else {
                    scegli_file(&files, &config);
                }
            }
            1 => {
                let nuovo_path: String = Input::new()
                    .with_prompt("Inserisci il nuovo percorso della cartella")
                    .interact_text()
                    .unwrap();

                path = nuovo_path.trim().to_string();
                config.cartella = path.clone();
                salva_config(&config);
            }
            2 => {
                println!("{}", "Uscita in corso...".red().bold());
                break;
            }
            _ => println!("Scelta non valida."),
        }
    }
}

pub fn scegli_file(files: &Vec<String>, config: &Config) {
    let mut opzioni = files.clone();
    opzioni.push("❌ Annulla".to_string());

    let scelta = Select::new()
        .items(&opzioni)
        .default(0)
        .interact()
        .unwrap();

    if scelta == files.len() {
        println!("Operazione annullata.");
        return;
    }

    let file_scelto = &files[scelta];

    if config.menu.mostra_descrizione {
        descrizione_file(file_scelto, config.menu.colori);
    }

    esegui_file(file_scelto);
}
