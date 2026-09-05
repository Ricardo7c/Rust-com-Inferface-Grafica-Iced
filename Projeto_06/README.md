
# Projeto 06: Semáforo Visual de Validação

## Objetivo

Praticar a aplicação de estilos visuais nativos no Iced utilizando funções auxiliares (`text::danger`, `button::success`, etc.) e estilização condicional de acordo com a validação do estado.

## Enunciado

Crie uma aplicação que contenha um campo de texto (`text_input`) e uma mensagem de feedback logo abaixo.
O usuário deve digitar uma senha ou código.

* Se o campo contiver menos de 5 caracteres, o texto deve exibir: `"Código muito curto (mínimo 5 caracteres)"` e ser renderizado com o estilo de erro (`text::danger`).


* Se o campo contiver 5 caracteres ou mais, o texto deve exibir: `"Código válido!"` e o botão `"Confirmar"` logo abaixo deve adotar o estilo de sucesso (`button::success`). Quando o código for inválido, o botão deve permanecer com o estilo padrão ou secundário.



## Requisitos

* Struct `App` contendo o campo de entrada (`codigo: String`) derivando `Default`.


* Enum `Message` para receber a digitação do campo.


* No método `view`:


* Usar `text_input` com placeholder e evento de digitação.


* Aplicar o método `.style(...)` no widget `text` de forma condicional para destacar a mensagem de erro quando o código tiver menos de 5 caracteres.


* Aplicar o método `.style(...)` no widget `button` para aplicar `button::success` quando o código tiver 5 ou mais caracteres.


* Centralizar a coluna na janela usando `container` e espaçamento.





## Entrada

Texto digitado no componente `text_input`.

## Saída

Uma interface que reage visualmente com alteração de cores e estilos pré-definidos (`text::danger` e `button::success`) conforme o tamanho do texto atinge o critério estabelecido.

## Exemplos

* **Ao abrir:** Campo vazio; texto exibe `"Código muito curto (mínimo 5 caracteres)"` em estilo vermelho/perigo (`text::danger`).


* **Digita "123":** Comprimento 3 (< 5); texto continua em `text::danger`.


* **Digita "12345":** Comprimento 5 (>= 5); texto passa para `"Código válido!"` com estilo regular e o botão `"Confirmar"` passa a ter destaque verde (`button::success`).



## Conceitos praticados

* Aplicação de estilos via funções utilitárias dos módulos de widgets (`button::success`, `text::danger`).


* Alternância de estilos em expressões baseadas no estado da aplicação.


* Inspeção do comprimento de `String` com o método `.len()` da biblioteca padrão.



## Dificuldade

🟢 Fácil
