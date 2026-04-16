Questa guida riassume e spiega in modo chiaro e strutturato tutto ciò che hai costruito finora nel tuo progetto Rust: un **gestore di script interattivo**, completo di menu, colori, descrizioni file e lancio degli script in nuove finestre.

---
## 1. Obiettivo del Progetto

Il progetto ha lo scopo di creare un **CLI tool professionale** che permetta di:

- Leggere una cartella contenente script `.bat`, `.cmd` e `.sh`.
- Mostrare gli script in un menu interattivo.
- Visualizzare informazioni dettagliate su ogni file.
- Eseguire gli script in una nuova finestra del terminale.
- Cambiare cartella dinamicamente.
- Navigare tramite un menu principale.

Il tutto con un'interfaccia colorata e moderna.

---
## 2. Struttura Generale del Programma

Il programma è stato suddiviso in funzioni modulari per garantire:

- leggibilità
- manutenibilità
- espandibilità

### Funzioni principali:

- `menu_principale()` → gestisce il loop e le scelte dell’utente.
- `leggi_file()` → legge la cartella e filtra gli script.
- `mostra_file()` → mostra i file trovati.
- `scegli_file()` → permette di selezionare un file tramite menu interattivo.
- `esegui_file()` → esegue lo script in una nuova finestra.
- `descrizione_file()` → mostra metadata e informazioni avanzate.
- Funzioni di supporto: `formatta_dimensione()`, `icona_file()`.

---
## 3. Lettura della Cartella e Filtraggio degli Script

La funzione `leggi_file()` utilizza:

- `fs::read_dir()` per leggere la directory.
- `path.extension()` per filtrare i file.
- Un `Vec<String>` per memorizzare i percorsi.

Sono supportati i formati:

- `.bat`
- `.cmd`
- `.sh`

---
## 4. Menu Principale Interattivo (Dialoguer)

Il menu principale utilizza la libreria **Dialoguer** per creare un’interfaccia navigabile con le frecce.
Il menu offre:

1. Lista script ed esecuzione
2. Cambio cartella
3. Uscita

Il menu rimane attivo grazie a un loop `loop { ... }`.

---
## 5. Scelta del File con Opzione "Annulla"

La funzione `scegli_file()` mostra:

- l’elenco degli script
- una voce aggiuntiva: **❌ Annulla**

Se l’utente seleziona Annulla, torna al menu principale.

---
## 6. Esecuzione degli Script

Gli script vengono eseguiti in una nuova finestra tramite:

### Per `.bat` e `.cmd`:

```
cmd /C start file.bat
```

### Per `.sh`:

```
bash file.sh
```

L’uso di `spawn()` permette di aprire una nuova finestra senza bloccare il programma Rust.

---

## 7. Colorazione dell’Interfaccia (Colored)

La libreria **colored** permette di colorare:

- titoli
- messaggi
- errori
- numeri

Esempi:

- `.red()`
- `.green()`
- `.yellow()`
- `.bold()`    

Questo rende l’interfaccia molto più leggibile.

---
## 8. Descrizione Avanzata dei File

La funzione `descrizione_file()` mostra:

- icona colorata in base al tipo di file
- dimensione formattata (B, KB, MB)
- data ultima modifica formattata con `chrono`

### Esempio di output:

```
=== DESCRIZIONE FILE ===
File: C:\script\deploy.sh
Tipo: 🐚
Dimensione: 3.42 KB
Ultima modifica: 16/04/2026 15:58:12
=========================
```

---

## 9. Librerie Utilizzate

### Dialoguer

Per menu interattivi:

- `Select`
- `Input`

### Colored

Per colorare il testo.

### Chrono

Per formattare date e orari.

### std::fs, std::io, std::process

Per:

- leggere file
- gestire input
- eseguire processi esterni
