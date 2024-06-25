use utils::terminal::{esperar_enter, exibe_menu, limpar_tela};

mod utils;

fn main() {
    let itens = ["Fundamentos", "Tipos", "Controle", "Funções", "Ownership"];
    let selecionado = exibe_menu("Título", &itens, true);
    esperar_enter();
}
