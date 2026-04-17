use std::fs;
use std::process::Command;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use colored::*;

use crate::utils::{formatta_dimensione, icona_file};

pub fn leggi_file(cartella: &str) -> Vec<String> {
    let entries = fs::read_dir(cartella).expect("Impossibile leggere la cartella");
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.expect("Errore nella lettura di un file");
        let path = entry.path();

        if let Some(ext) = path.extension() {
            let estensione = ext.to_string_lossy();
            if estensione == "bat" || estensione == "cmd" || estensione == "sh" {
                files.push(path.display().to_string());
            }
        }
    }

    files
}

pub fn esegui_file(percorso: &str) {
    let estensione = std::path::Path::new(percorso)
        .extension()
        .unwrap()
        .to_string_lossy()
        .to_lowercase();

    println!("\nEseguo il file...");

    if estensione == "bat" || estensione == "cmd" {
        Command::new("cmd")
            .args(["/C", "start", percorso])
            .spawn()
            .expect("Errore nell'esecuzione del file .bat/.cmd");
    } else if estensione == "sh" {
        Command::new("bash")
            .arg(percorso)
            .spawn()
            .expect("Errore nell'esecuzione del file .sh");
    } else {
        println!("{}", "Tipo di file non supportato".red());
    }
}

pub fn descrizione_file(percorso: &str, usa_colori: bool) {
    let metadata = match fs::metadata(percorso) {
        Ok(m) => m,
        Err(_) => {
            println!("{}", "Impossibile leggere i metadati del file".red());
            return;
        }
    };

    let dimensione = formatta_dimensione(metadata.len());
    let modificato = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modificato.into();
    let data_formattata = datetime.format("%d/%m/%Y %H:%M:%S").to_string();
    let icona = icona_file(percorso);

    if usa_colori {
        println!("\n{}", "=== DESCRIZIONE FILE ===".bold().blue());
        println!("{} {}", "File:".bold(), percorso.yellow());
        println!("{} {}", "Tipo:".bold(), icona);
        println!("{} {}", "Dimensione:".bold(), dimensione.green());
        println!("{} {}", "Ultima modifica:".bold(), data_formattata.cyan());
        println!("{}", "=========================".bold().blue());
    } else {
        println!("\n=== DESCRIZIONE FILE ===");
        println!("File: {}", percorso);
        println!("Tipo: {}", icona);
        println!("Dimensione: {}", dimensione);
        println!("Ultima modifica: {}", data_formattata);
        println!("=========================");
    }
}
