# Projeto 07: Seletor de Três Temas

## Objetivo

Configurar e alternar dinamicamente temas globais da aplicação no Iced utilizando o construtor `.theme(...)`, manipulando variantes em um enum de estado customizado.

## Enunciado

Crie uma interface que contenha um título, um subtítulo e três botões lado a lado (`row!`).
Cada botão deve ativar um tema global diferente da biblioteca Iced:

1. `"Claro"` -> ativa `Theme::Light`

2. `"Escuro"` -> ativa `Theme::Dark`

3. `"Tokyo Night"` -> ativa `Theme::TokyoNight`


O tema escolhido deve ser refletido instantaneamente em toda a janela.

## Requisitos

* Criar a aplicação utilizando o builder `iced::application` conectando o método `theme` via `.theme(...)`.


* Armazenar o tema selecionado no estado da struct `App` (pode guardar diretamente `Theme` ou um enum próprio).


* Criar variantes no enum `Message` para os três botões de seleção de tema.


* No método `update`, atualizar o tema ativo de acordo com o botão clicado.


* Na função `view`, exibir os botões alinhados horizontalmente em um `row!` dentro de uma `column!` centralizada.


* Implementar o método de tema retornando o `Theme` ativo.



## Entrada

Cliques do usuário nos botões de seleção de tema.

## Saída

A janela e seus componentes mudando a paleta de cores global para o tema correspondente ao botão clicado.

## Exemplos

* **Ao abrir:** Aplicação inicia com o tema padrão definido no `Default`.
* **Clique em "Tokyo Night":** Toda a interface adota as cores de `Theme::TokyoNight`.


* **Clique em "Claro":** Toda a interface adota as cores de `Theme::Light`.

## Conceitos praticados

* Configuração do builder `.theme(...)` em `iced::application`.


* Manipulação de `iced::Theme` no estado.


* Layout com `row!` e `column!` combinados.



## Dificuldade

🟢 Fácil
