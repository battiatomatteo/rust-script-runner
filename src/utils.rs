use colored::*;

pub fn formatta_dimensione(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    }
}

pub fn icona_file(percorso: &str) -> String {
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
