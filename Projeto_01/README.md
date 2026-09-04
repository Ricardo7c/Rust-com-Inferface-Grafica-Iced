# Projeto 01: Hello World

## Objetivo

Criar a aplicação gráfica mais simples possível utilizando a biblioteca Iced, aprendendo a abrir uma janela, renderizar texto com o widget `text` e centralizar o conteúdo na tela utilizando `container` e estratégias de dimensionamento `Fill`.

## Enunciado

Construa uma aplicação desktop mínima contendo apenas uma janela gráfica. No centro exato dessa janela, tanto horizontal quanto verticalmente, exiba a mensagem: `"Hello, World!"`. Como a tela é estática e não possui botões ou campos de entrada, nenhuma mensagem precisa alterar o estado da aplicação.

## Requisitos

* Inicialização da aplicação:
* Definir o ponto de entrada `pub fn main() -> iced::Result` e executar a interface gráfica utilizando `iced::run`.




* Funções fundamentais da arquitetura:
* Implementar uma função `update` com a assinatura necessária para satisfazer o contrato esperado por `iced::run`.


* Implementar a função `view` que constrói os elementos visuais e retorna um `Element` genérico do Iced.




* Layout e componentes visuais:
* Utilizar o widget `text("Hello, World!")` para renderizar o texto na tela.


* Envolver o texto em um widget `container`.


* Configurar o `container` para preencher todo o espaço disponível na janela com `Fill` e aplicar o alinhamento centralizado tanto no eixo horizontal (`center_x`) quanto no vertical (`center_y`).





## Entrada

Nenhuma entrada de dados ou evento de interação do usuário é necessário.

## Saída

Uma janela de desktop contendo a frase `"Hello, World!"` posicionada exatamente no centro. Redimensionar a janela deve manter o texto perfeitamente centralizado.

## Exemplos

* **Ao executar:**
A janela abre imediatamente com fundo padrão do tema e a inscrição central:
```text
┌───────────────────────────────┐
│                               │
│                               │
│         Hello, World!         │
│                               │
│                               │
└───────────────────────────────┘

```



## Conceitos praticados

* Ponto de entrada e ciclo de vida básico de uma aplicação Iced com `iced::run`.


* Tipos de retorno: `Element<'_, Message>` e conversão com `.into()`.


* Layout, dimensionamento espacial com `Fill` e alinhamento usando o widget `container`.


* Estruturação de código sem estado interativo inicial.

## Dificuldade

🟢 Fácil