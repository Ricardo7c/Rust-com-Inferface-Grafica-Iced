
# Projeto 02: Interruptor de Mensagem (Toggle Text)

## Objetivo

Aprender a conectar uma ação da interface ao ciclo de vida do Iced: emitir um evento via clique de botão, atualizar o estado da aplicação com um booleano e refletir a mudança no texto renderizado.

## Enunciado

Crie uma aplicação que exiba um texto e um botão logo abaixo.
Inicialmente, o texto deve mostrar `"Estado: Desligado"`. Ao clicar no botão, o texto deve mudar para `"Estado: Ligado"`. Se o usuário clicar novamente, o texto deve voltar para `"Estado: Desligado"`, alternando a cada clique.

## Requisitos

* Estruturar o estado da aplicação em uma `struct` contendo um campo booleano (ex: `ligado: bool`).


* Implementar ou derivar `Default` para iniciar o booleano como `false`.


* Declarar um enum `Message` com uma variante que represente o ato de alternar.


* Implementar o método `update` recebendo `&mut self` e invertendo o valor do booleano quando a mensagem for recebida.


* Implementar o método `view` retornando `Element<'_, Message>`:


* Usar o widget `column!` ou `Column` para empilhar o texto e o botão verticalmente.


* Usar o widget `button` com o método `.on_press(...)` associado à mensagem do enum.


* Centralizar a coluna na janela utilizando `container`.





## Entrada

Cliques do usuário no botão.

## Saída

Uma janela desktop com a coluna centralizada exibindo o texto atualizado (`"Estado: Ligado"` ou `"Estado: Desligado"`) conforme os cliques forem executados.

## Exemplos

* **Ao abrir:** Texto exibe `"Estado: Desligado"`.
* **Clique 1:** Texto exibe `"Estado: Ligado"`.
* **Clique 2:** Texto exibe `"Estado: Desligado"`.
* **Clique 3:** Texto exibe `"Estado: Ligado"`.

## Conceitos praticados

* Manipulação de estado com `bool` e operador de negação `!`.


* Emissão de eventos com `.on_press(Message)` no widget `button`.


* Layout vertical com o widget/macro `column!` ou `Column` e espaçamento.


* Comunicação entre View e Update via enum.



## Dificuldade

🟢 Fácil

