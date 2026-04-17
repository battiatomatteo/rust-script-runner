use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub cartella: String,
    pub menu: MenuConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MenuConfig {
    pub colori: bool,
    pub mostra_descrizione: bool,
}

pub fn carica_config() -> Config {
    let contenuto = std::fs::read_to_string("config.toml")
        .expect("Impossibile leggere config.toml");

    toml::from_str(&contenuto)
        .expect("Errore nel parsing del file TOML")
}

pub fn salva_config(config: &Config) {
    let toml_string = toml::to_string(config)
        .expect("Errore nella conversione in TOML");

    std::fs::write("config.toml", toml_string)
        .expect("Impossibile salvare config.toml");
}