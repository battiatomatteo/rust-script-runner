// fs::read_dir("percorso")   --> Questa restituisce un iteratore con tutti gli elementi della cartella.
use std::fs;
use std::io;
use std::process::Command;

fn main() {
    // 1. percorso della cartella da leggere 
    let path = "C:\\Users\\baddy\\Desktop\\progetto_rust\\script_bat";

    println!("\nLeggo i file .bat, .cmd e .sh nella cartella {}:", path);

    // 2. leggo la directory
    let entries = fs::read_dir(path).expect("Impossibile leggere la cartella");  // restituisce un iteratore, se c'è un errore, stampa il messaggio
    
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
                println!("- {}", path.display()); // stampo il percorso del file
                files.push(path.display().to_string()); // aggiungo il percorso del file al vettore, to_string() converte il percorso in stringa
            }
        }
    }


    // 5. Stampo il numero di file trovati
    println!("\n---- File trovati ----");
    // .iter() è un metodo che restituisce un iteratore su un vettore, 
    // .enumerate() restituisce un iteratore con indice, i è l'indice, file è il percorso del file
    //for(i, file) in files.iter().enumerate(){ // iteratore con indice, i è l'indice, file è il percorso del file
        //println!("{}: {}", i+1, file); // stampo il numero e il percorso del file
    //}

    /*
    Come funziona la lettura dell’input in Rust:
        std::io::stdin() → per leggere dal terminale
        read_line() → per prendere il testo
        trim() → per togliere spazi e newline
        parse::<u32>() → per convertire in numero
     */

    // 6. Chiedo all'utente di scegliere un file da eseguire 
    println!("\nScegli un file da eseguire dalla lista (inserisci il numero corrispondente):");

    let mut scelta = String::new();
    io::stdin().read_line(&mut scelta).expect("Errore nella lettura dell'input"); // leggo l'input dell'utente, se c'è un errore, stampa il messaggio

    let scelta_num: usize = scelta.trim().parse().expect("Per favore, inserisci un numero valido"); // converto l'input in numero, se c'è un errore, stampa il messaggio

    // convertiamo il numero in indice 
    let indice = scelta_num -1; // l'indice parte da 0, quindi sottraiamo 1

    // recupero il file
    let file_scelto = &files[indice]; // prendo il percorso del file scelto

    println!("\nHai scelto di eseguire: {}", file_scelto); // stampo il percorso del file scelto

    // 7. Eseguo il file scelto
    let estensione = std::path::Path::new(file_scelto)
        .extension()
        .unwrap()
        .to_string_lossy()
        .to_lowercase();

    println!("\nEseguo il file...");

    // Eseguiamo il file in base alla sua estensione

    if estensione == "bat" || estensione == "cmd" {
        Command::new("cmd")
            .args(["/C", "start", file_scelto])
            .spawn()
            .expect("Errore nell'esecuzione del file .bat/.cmd");
    } else if estensione == "sh" {
        Command::new("bash")
            .arg(file_scelto)
            .spawn()
            .expect("Errore nell'esecuzione del file .sh");
    } else {
        println!("Tipo di file non supportato");
    }
    
}



/*
Rust usa il modulo:
 std::fs → per leggere directory e file
 std::path → per lavorare con i percorsi
*/