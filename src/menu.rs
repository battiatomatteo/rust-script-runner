use colored::*;
use dialoguer::{Select, Input};
use crate::config::{Config, salva_config};
use crate::file_ops::{leggi_file, descrizione_file, esegui_file, mostra_info};

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
            "Impostazioni",
            "Informazioni",
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
            2 => menu_impostazioni(&mut config),
            3 => mostra_info(config.menu.colori),
            4 => { println!("Uscita..."); break; }
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

pub fn menu_impostazioni(config: &mut Config) {
    let opzioni = vec![
        "Attiva/Disattiva colori",
        "Mostra/Nascondi descrizione file",
        "Torna al menu principale",
    ];

    let scelta = Select::new()
        .with_prompt("Impostazioni")
        .items(&opzioni)
        .default(0)
        .interact()
        .unwrap();

    match scelta {
        0 => {
            config.menu.colori = !config.menu.colori;
            salva_config(config);
            println!("Colori: {}", config.menu.colori);
        }
        1 => {
            config.menu.mostra_descrizione = !config.menu.mostra_descrizione;
            salva_config(config);
            println!("Descrizione file: {}", config.menu.mostra_descrizione);
        }
        2 => return,
        _ => {}
    }
}