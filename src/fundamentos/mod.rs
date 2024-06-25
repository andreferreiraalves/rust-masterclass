use crate::utils::terminal::{esperar_enter, exibe_menu, limpar_tela};

mod primeiro;
mod variaveis;

pub fn executar() {
    loop {
        let itens = [
            "Primeiro Exemplo",
            "Variáveis - Imutáveis",
            "Variáveis - Mutáveis",
            "Variáveis - Constantes",
            "Variáveis - Shadowing",
            "Operadores - Aritméticos",
            "Operadores - Relacionais",
            "Operadores - Lógicos",
        ];

        let selecionado = exibe_menu("Fundamentos", &itens, false);

        limpar_tela();

        match selecionado {
            1 => primeiro::exemplo(),
            2 => variaveis::imutaveis(),
            3 => variaveis::mutaveis(),
            4 => variaveis::constantes(),
            5 => variaveis::shadowing(),
            _ => break,
        };

        esperar_enter();
    }
}
