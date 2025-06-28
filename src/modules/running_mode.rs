use crate::utils::ui;

pub fn configure_search() -> Option<String> {
    println!("\n🔍 Mode de récupération :");
    println!("  1️⃣  Filtrer par mot-clé");
    println!("  2️⃣  500 derniers repos");

    let mode = loop {
        match ui::prompt("\n👉 Choix (1 ou 2) : ").as_str() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("❌ Saisissez 1 ou 2"),
        }
    };

    let result = match mode {
        1 => {
            let keyword = loop {
                let input = ui::prompt("\n🔎 Mot-clé : ");
                if !input.is_empty() {
                    break input.to_lowercase();
                }
                println!("❌ Mot-clé requis");
            };
            Some(keyword)
        }
        _ => None,
    };

    ui::clear_menu_lines(8);

    result
}
