use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct ProgressTracker {
    current_step: usize,
    total_steps: usize,
}

pub struct ProgressBar {
    total: usize,
    current: Arc<AtomicUsize>,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: Arc::new(AtomicUsize::new(0)),
            width: 50,
        }
    }

    pub fn get_counter(&self) -> Arc<AtomicUsize> {
        self.current.clone()
    }

    pub fn update(&self) {
        let current = self.current.load(Ordering::Relaxed);
        let percent = if self.total > 0 {
            (current * 100) / self.total
        } else {
            0
        };
        let filled = (current * self.width) / self.total.max(1);

        print!("\r   ");
        print!("[");

        for i in 0..self.width {
            if i < filled {
                print!("█");
            } else {
                print!("░");
            }
        }

        print!("] {}% ({}/{})", percent, current, self.total);
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        self.update();
        println!();
    }
}

impl ProgressTracker {
    pub fn new(total_steps: usize) -> Self {
        Self {
            current_step: 0,
            total_steps,
        }
    }

    pub fn start_step(&mut self, message: &str) {
        self.current_step += 1;
        print!("{}/{} {}...", self.current_step, self.total_steps, message);
        io::stdout().flush().unwrap();
    }

    pub fn complete_step(&mut self, message: &str, details: &str) {
        println!(
            "\r{}/{} {}... ✅ ({})",
            self.current_step, self.total_steps, message, details
        );
    }

    pub fn complete_step_with_status(&mut self, message: &str, status: &str) {
        println!(
            "\r{}/{} {}... {}",
            self.current_step, self.total_steps, message, status
        );
    }

    pub fn show_info(&self, message: &str) {
        println!("   {}", message);
    }

    pub fn finalize(&self) {
        println!("{}/{} Terminé ! ✅", self.total_steps, self.total_steps);
    }
}

pub fn print_header() {
    println!("\n");
    println!("                       🚀 AspiGro                         ");
    println!("==========================================================");
    println!("       l'outil pour aspirer des repos comme un pro !      ");
    println!("==========================================================");
    println!("\n");
}

/// Efface les lignes du menu interactif pour un affichage propre
pub fn clear_menu_lines(lines: usize) {
    print!("\x1b[{}A\x1b[0J", lines);
    io::stdout().flush().unwrap();
}

pub fn prompt(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur saisie");
    input.trim().to_string()
}
