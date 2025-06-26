mod load_env;
mod running_mode;

fn main() {
    println!("\n                🚀 AspiGro                 ");
    println!("==========================================================");
    println!("l'outil pour aspirer des repos comme un pro !");
    println!("==========================================================\n");

    load_env::load_github_token();

    running_mode::chose_mode();
}
