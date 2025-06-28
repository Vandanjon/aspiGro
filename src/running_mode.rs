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

pub fn configure_search() -> Option<String> {
    println!("\n🔍 Mode de récupération :");
    println!("  1️⃣  Manuel : Filtrer par mot-clé dans le nom du repository");
    println!("  2️⃣  Auto : Récupérer les 500 derniers repositories (par date de push)");

    let mode = loop {
        let choice = get_user_input("\n👉 Votre choix (1 ou 2) : ");
        match choice.as_str() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("❌ Veuillez saisir 1 ou 2"),
        }
    };

    if mode == 1 {
        println!("✅ Vous avez choisi le mode 1 : Filtrer par mot-clé");

        let keyword = loop {
            let input =
                get_user_input("\n🔎 Mot-clé à rechercher dans les noms de repositories : ");
            if input.trim().is_empty() {
                println!("❌ Le mot-clé ne peut pas être vide");
                continue;
            }
            break input.trim().to_lowercase();
        };

        println!("✅ Mot-clé sélectionné : '{}'", keyword);

        Some(keyword)
    } else {
        println!("✅ Vous avez choisi le mode 2 : Récupérer les 500 derniers repositories");

        None
    }
}
