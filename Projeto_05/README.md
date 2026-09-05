# Projeto 05: Dobrador de Números (Parsing Seguro com Result)

## Objetivo

Praticar a conversão de tipos de `String` para inteiro (`i32`) com tratamento de falhas usando `Result` e *pattern matching*, exibindo feedback visual em tempo real para o usuário.

## Enunciado

Construa uma aplicação com um único campo `text_input` e um texto logo abaixo.
O usuário deve digitar um número inteiro no campo. A aplicação deve tentar converter o texto digitado para `i32`:

* Se o campo estiver vazio, exiba: `"Digite um número inteiro."`
* Se o valor for um número válido, calcule o seu dobro e exiba: `"O dobro de <N> é <2 * N>."`
* Se o usuário digitar algo que não seja um número (por exemplo, letras como `"abc"` ou símbolos), exiba: `"Erro: valor digitado não é um número válido!"`


## Requisitos

* Modelar o estado da struct `App` armazenando o texto do campo (`entrada: String`).


* Criar uma variante no enum `Message` para receber a entrada do usuário (`text_input` com `.on_input(...)`).


* Atualizar a string no método `update`.


* No método `view`:


* Chamar `.trim()` e `.parse::<i32>()` sobre a string digitada.


* Tratar o retorno do tipo `Result<i32, _>` utilizando `match` ou construções como `if let Ok(...)` / `if let Err(...)`.


* Renderizar a mensagem correspondente dependendo do resultado do parse.


* Organizar os componentes em uma coluna vertical centralizada (`column!` e `container`).





## Entrada

Caracteres digitados pelo usuário no campo `text_input`.

## Saída

Um texto na janela que reflete em tempo real o cálculo do dobro ou uma mensagem informativa de validação.

## Exemplos

* **Ao abrir:** Campo vazio; texto exibe `"Digite um número inteiro."`
* **Digita "7":** Texto exibe `"O dobro de 7 é 14."`
* **Digita "-3":** Texto exibe `"O dobro de -3 é -6."`
* **Digita "dez":** Texto exibe `"Erro: valor digitado não é um número válido!"`


## Conceitos praticados

* Método `.parse::<T>()` disponível para fatias de texto (`&str`).


* Tratamento de erros com `Result<T, E>` (`Ok(val)` e `Err(e)`).


* Controle de fluxo em Rust para validação de dados antes de processamento aritmético.



## Dificuldade

🟢 Fácil
