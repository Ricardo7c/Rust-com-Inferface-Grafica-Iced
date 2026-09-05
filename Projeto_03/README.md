# Título

Espelho de Texto (Eco de Digitação)

## Objetivo

Aprender a capturar dados digitados pelo usuário em tempo real utilizando o widget `text_input`, atualizando dinamicamente uma `String` no estado e refletindo essa entrada imediatamente em outro elemento da interface.

## Enunciado

Construa uma aplicação com um campo de digitação e um texto logo abaixo.
À medida que o usuário digitar qualquer caractere no campo, o texto abaixo deve atualizar imediatamente, exibindo exatamente o que está escrito no campo precedido por `"Você digitou: "`. Quando o campo estiver vazio, o texto deve exibir `"Você digitou: (nada ainda)"`.

## Requisitos

* Estruturar o estado com um campo de texto (ex: `entrada: String`).


* Declarar uma variante no enum `Message` que transporte o conteúdo atualizado da digitação (ex: `InputChanged(String)`).


* No método `update`, capturar essa `String` via *pattern matching* e substituir o valor salvo no estado.


* No método `view`:


* Importar e utilizar o widget `text_input` do módulo `iced::widget`.


* Configurar o `text_input` com:
* Um placeholder textual para orientação (ex: `"Digite algo..."`).


* O valor atual do estado vinculado ao campo.


* O manipulador de evento `.on_input(Message::InputChanged)`.




* Renderizar abaixo o texto com o prefixo `"Você digitou: "` seguido do conteúdo ou de `"(nada ainda)"` quando a `String` estiver vazia.


* Organizar os elementos em uma coluna vertical com espaçamento e centralizá-la na janela.





## Entrada

Digitação de caracteres pelo usuário no componente `text_input`.

## Saída

Uma janela desktop com layout centralizado onde o texto abaixo reflete, em tempo real, a cadeia de caracteres presente no campo de digitação.

## Exemplos

* **Ao abrir o app:** Campo com placeholder `"Digite algo..."`; texto exibe `"Você digitou: (nada ainda)"`.


* **Digita "Rust":** Campo contém `"Rust"`; texto exibe `"Você digitou: Rust"`.
* **Apaga tudo:** Campo vazio; texto volta a exibir `"Você digitou: (nada ainda)"`.

## Conceitos praticados

* Widget `text_input` e seu método `.on_input(...)`.


* Variantes de `enum` que carregam dados (`Message::NomeDaVariante(String)`).


* Transferência de dados via *pattern matching* destruturador no `update`.


* Formatação de texto com a macro `format!` da biblioteca padrão do Rust.



## Dificuldade

🟢 Fácil

---

Tente implementar seguindo a mesma estrutura do exercício anterior e envie seu código quando estiver pronto! Se surgir qualquer dúvida sobre a assinatura do `text_input` ou como variantes com dados funcionam no enum, basta perguntar.