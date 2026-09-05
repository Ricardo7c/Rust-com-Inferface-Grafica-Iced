use iced::Alignment::Center;
use iced::widget::{column, container, row, text, text_input};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .run()
}

#[derive(Default)]
struct App {
    // Estados aqui
    nome: String,
    sobrenome: String
}

#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    Nome(String),
    Sobrenome(String)
}

impl App {
    fn title(&self) -> String {
        String::from("Gerador de Crachá")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Nome(nome) => self.nome = nome,
            Message::Sobrenome(sobrenome) => self.sobrenome = sobrenome
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let texto = if !self.nome.is_empty() || !self.sobrenome.is_empty(){
            format!("Crachá: {} {}", self.nome, self.sobrenome)
        }else{
            "Crachá: (informe nome e sobrenome)".to_string()
        };

        let content = column![
            text(texto).size(20),

            row![
                text("Nome: ").size(20),
                text_input("Digite seu nome...", &self.nome).on_input(Message::Nome)
            ].align_y(Center),
            row![
                text("Sobrenome: ").size(20),
                text_input("Digite seu sobrenome...", &self.sobrenome).on_input(Message::Sobrenome)
            ].align_y(Center)
        ].spacing(20).align_x(Center).width(500);

        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}