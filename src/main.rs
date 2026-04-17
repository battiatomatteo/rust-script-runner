// fs::read_dir("percorso")   --> Questa restituisce un iteratore con tutti gli elementi della cartella.
use std::fs;
//use std::io;
use std::process::Command;
use colored::*;
use dialoguer::{Select, Input};
use std::time::{SystemTime};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    cartella: String,
    menu: MenuConfig,
}

#[derive(Deserialize, Serialize, Debug)]
struct MenuConfig {
    colori: bool,
    mostra_descrizione: bool,
}

fn carica_config() -> Config {
    let contenuto = std::fs::read_to_string("config.toml")
        .expect("Impossibile leggere config.toml");

    toml::from_str(&contenuto)
        .expect("Errore nel parsing del file TOML")
}

fn salva_config(config: &Config) {
    let toml_string = toml::to_string(config).expect("Errore nella conversione in TOML");

    std::fs::write("config.toml", toml_string)
        .expect("Impossibile salvare config.toml");
}

fn menu_principale (){
    let mut config = carica_config();   // 0. carico la configurazione dal file config.toml, se il file non esiste, viene creato con i valori di default

    // 1. percorso della cartella da leggere 
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
        
        //print!("{} ", "Scelta: ".bold().green());

        match scelta {  // match è usato per confrontare il valore di scelta con i casi specificati, trim() rimuove gli spazi bianchi
            0 => {
                let files = leggi_file(&path);
                if files.is_empty() {
                    println!("Nessun file trovato .");
                }else {
                    scegli_file(&files, &config, config.menu.colori);
                }
            }
            1 => {
                let nuovo_path: String = Input::new()
                    .with_prompt("Inserisci il nuovo percorso della cartella")
                    .interact_text()
                    .unwrap();

                path = nuovo_path.trim().to_string();

                // Salvo nel TOML
                config.cartella = path.clone();
                salva_config(&config);
            }
            2 => {
                println!("{}", "Uscita in corso...".red().bold());
                break;
            }
            _ => println!("{}", "Scelta non valida, riprova.".red()),
        }
    }
}

fn scegli_file(files: &Vec<String>, config: &Config, usa_colori: bool) {
    println!("{}", "\nInserisci il numero del file da eseguire:".bold().yellow());

    let mut opzioni = files.clone();  // creo una copia del vec così da poter inserire l'opzione di annullamento
    opzioni.push("❌ Annulla".to_string());

    let scelta = Select::new()
        .items(&opzioni)
        .default(0)
        .interact()
        .unwrap();

    // Se l’utente ha scelto "Annulla"
    if scelta == files.len() {
        println!("{}", "Operazione annullata. Ritorno al menu principale.".cyan());
        return;
    }

    let file_scelto = &files[scelta];
    if usa_colori {
         println!("{} {}", "Hai scelto:".green(), file_scelto.bold());
    } else {
        println!("Hai scelto: {}", file_scelto);
    }

    if config.menu.mostra_descrizione {
        descrizione_file(file_scelto, config.menu.colori);
    }


    esegui_file(file_scelto);
}


fn leggi_file( cartella: &str) -> Vec<String> {     // 2. leggo la directory
    let entries = fs::read_dir(cartella).expect("Impossibile leggere la cartella");  // restituisce un iteratore, se c'è un errore, stampa il messaggio
    // creo un vettore per memorizzare i file trovati ( mut perchè così lo posso modificare)
    let mut files: Vec<String> = Vec::new();  // Vec è un vettore dinamico, String è una stringa dinamica, Vec::new() crea un nuovo vettore vuoto

    // 3. Itero su ogni elemento della cartella
    for entry in entries { // entry è un Risultato , 
        let entry = entry.expect("Errore nella lettura di un file"); // se c'è un errore, stampa il messaggio
        let path = entry.path(); // prendo il percorso del file

        // 4. Controllo se il file ha estensione .bat
        if let Some(ext) = path.extension() { // se il file ha un'estensione
            let estensione = ext.to_string_lossy(); // converto l'estensione in stringa, to_string_lossy() è usato per gestire eventuali errori di codifica

            if estensione == "bat" || estensione == "cmd" || estensione == "sh" { // se l'estensione è .bat o .cmd o .sh
                //println!("- {}", path.display()); // stampo il percorso del file
                files.push(path.display().to_string()); // aggiungo il percorso del file al vettore, to_string() converte il percorso in stringa
            }
        }
    }

    files
}

fn esegui_file(percoeso: &str) {    // 7. Eseguo il file scelto
    let estensione = std::path::Path::new(percoeso)
        .extension()
        .unwrap()
        .to_string_lossy()
        .to_lowercase();

    println!("\nEseguo il file...");

    // Eseguiamo il file in base alla sua estensione

    if estensione == "bat" || estensione == "cmd" {
        Command::new("cmd")
            .args(["/C", "start", percoeso])
            .spawn()
            .expect("Errore nell'esecuzione del file .bat/.cmd");
    } else if estensione == "sh" {
        Command::new("bash")
            .arg(percoeso)
            .spawn()
            .expect("Errore nell'esecuzione del file .sh");
    } else {
        println!("{}", "Tipo di file non supportato".red());
    }
}

// funzione per descrivere il file scelto, mostra la dimensione, l'ultima modifica e un'icona in base all'estensione del file
fn descrizione_file(percorso: &str, usa_colori: bool) {  
    let metadata = match fs::metadata(percorso) {
        Ok(m) => m,
        Err(_) => {
            if usa_colori{
                println!("{}", "Impossibile leggere i metadati del file".red());
            } else {
                println!("Impossibile leggere i metadati del file");
            }
            return;
        }
    };

    // Dimensione file 
    let dimensione = formatta_dimensione(metadata.len());

    // Ultima modifica
    let modificato = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = modificato.into();
    let data_formattata = datetime.format("%d/%m/%Y %H:%M:%S").to_string();

    // Icona
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
        println!("{} {}", "Tipo:", icona);
        println!("{} {}", "Dimensione:", dimensione);
        println!("{} {}", "Ultima modifica:", data_formattata);
        println!("{}", "=========================");
    }

    
}

// funzione per formattare la dimensione del file in modo leggibile, ad esempio 1.5 MB invece di 1572864 B
// questa funzione è stata presa da internet, non è stata scritta da me, è stata modificata per adattarla al mio progetto
fn formatta_dimensione(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    }
}

// funzione per restituire un'icona in base all'estensione del file, utilizza colored per colorare l'icona
// questa funzione è stata presa da internet, non è stata scritta da me, è stata modificata per adattarla al mio progetto
fn icona_file(percorso: &str) -> String {
    let est = std::path::Path::new(percorso)
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    match est.as_str() {
        "bat" => "⚙️".green().bold().to_string(),
        "cmd" => "📄".cyan().bold().to_string(),
        "sh"  => "🐚".yellow().bold().to_string(),
        _     => "❓".red().bold().to_string(),
    }
}


fn main() {
    menu_principale();
}