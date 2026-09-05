# Projeto 04: Gerador de Crachá (Dois Campos de Entrada)

## Objetivo

Praticar a manipulação de múltiplos campos `text_input` no mesmo formulário, distinguindo as mensagens no `update` e combinando diferentes variáveis de estado para renderizar uma única saída formatada.

## Enunciado

Construa uma aplicação de criação de crachá virtual. A tela deve conter dois campos de entrada de texto empilhados: um para o **Nome** e outro para o **Sobrenome**.

Abaixo dos campos, deve haver um mostrador textual que exibe a concatenação desses dois valores.

* Se ambos os campos estiverem vazios, exiba: `"Crachá: (informe nome e sobrenome)"`.


* Caso contrário, exiba o nome e o sobrenome lado a lado: `"Crachá: <Nome> <Sobrenome>"`.



## Requisitos

* Modelar o estado em uma struct contendo dois campos independentes do tipo `String` (ex: `nome: String` e `sobrenome: String`).


* Criar um enum `Message` contendo **duas variantes**, cada uma carregando a `String` digitada pelo usuário no campo correspondente.


* Implementar o método `update` tratando as duas variantes separadamente via `match` para atualizar o campo de estado correto.


* No método `view`:


* Renderizar dois widgets `text_input`, cada um com seu próprio placeholder e conectado à sua variante do enum.


* Determinar o texto a ser exibido verificando se os dois campos estão vazios.


* Organizar os widgets em uma coluna (`column!`) com espaçamento (`.spacing(...)`) e largura definida, centralizando-a na janela com `container`.





## Entrada

Texto digitado pelo usuário em qualquer um dos dois campos de entrada (`text_input`).

## Saída

Uma interface que reflete em tempo real o nome completo formado pela combinação dos dois campos no texto de exibição.

## Exemplos

* **Ao iniciar:** Ambos os campos vazios; texto exibe `"Crachá: (informe nome e sobrenome)"`.


* **Digita "Ana" no primeiro campo:** Texto exibe `"Crachá: Ana "`.
* **Digita "Silva" no segundo campo:** Texto exibe `"Crachá: Ana Silva"`.
* **Apaga o conteúdo de ambos os campos:** Texto volta a exibir `"Crachá: (informe nome e sobrenome)"`.



## Conceitos praticados

* Múltiplos manipuladores de evento no enum `Message`.


* Gerenciamento de múltiplos campos `String` no mesmo estado.


* Composição de layout e alinhamento com widgets do Iced.



## Dificuldade

🟢 Fácil
