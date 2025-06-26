use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture de l'entrée");
    input.trim().to_string()
}

pub fn chose_mode() -> u8 {
    println!("\n🔍 Mode de récupération :");
    println!("  1️⃣  Filtrer par mot-clé dans le nom du repo");
    println!("  2️⃣  Récupérer les 500 meilleurs repos (par date de push)");

    loop {
        let choice = get_user_input("\n👉 Votre choix (1 ou 2) : ");
        match choice.as_str() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("❌ Veuillez saisir 1 ou 2"),
        }
    }
}
