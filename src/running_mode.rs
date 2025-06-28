use std::io::{self, Write};

/// Saisie utilisateur simplifiée
fn prompt(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur saisie");
    input.trim().to_string()
}

pub fn configure_search() -> Option<String> {
    println!("\n🔍 Mode de récupération :");
    println!("  1️⃣  Filtrer par mot-clé");
    println!("  2️⃣  500 derniers repos");

    // Sélection du mode avec pattern matching
    let mode = loop {
        match prompt("\n👉 Choix (1 ou 2) : ").as_str() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("❌ Saisissez 1 ou 2"),
        }
    };

    match mode {
        1 => {
            println!("✅ Mode filtrage par mot-clé");
            let keyword = loop {
                let input = prompt("\n🔎 Mot-clé : ");
                if !input.is_empty() {
                    break input.to_lowercase();
                }
                println!("❌ Mot-clé requis");
            };
            println!("✅ Mot-clé : '{}'", keyword);
            Some(keyword)
        }
        _ => {
            println!("✅ Mode 500 derniers repos");
            None
        }
    }
}
