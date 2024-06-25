use std::process::exit;

use utils::terminal::{exibe_menu, limpar_tela};

mod fundamentos;
mod utils;

fn main() {
    loop {
        let itens = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
        let selecionado = exibe_menu("Título", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::executar(),
            2 => println!("opt {}", 2),
            3 => println!("opt {}", 3),
            4 => println!("opt {}", 4),
            5 => println!("opt {}", 5),
            _ => exit(0),
        };
    }
}
