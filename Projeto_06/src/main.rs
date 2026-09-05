use iced::Alignment::Center;
use iced::widget::{button, column, container, text, text_input};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .run()
}

#[derive(Default)]
struct App {
    // Estados aqui
    senha: String
}

#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    Senha(String),
    Login
}

impl App {
    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Senha(senha) => self.senha = senha,
            Message::Login => println!("Login realizado com sucesso!")
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let (mensagem, botao) = match self.senha.len() {
            0..=4 => (
                text("Código muito curto (mínimo 5 caracteres)").style(text::danger),
                button("Login").style(button::secondary),
            ),
            _ => (
                text("Código válido!").style(text::default),
                button("Login").on_press(Message::Login).style(button::success),
            ),
        };

        
        let content = column![
            text_input("Digite sua senha", &self.senha).on_input(Message::Senha),
            mensagem,
            botao,
        ].spacing(20).width(500).align_x(Center);
        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}