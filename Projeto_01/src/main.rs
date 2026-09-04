use iced::widget::{container, text};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

#[derive(Default)]
struct App {
    // Estados aqui
}

#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
}

impl App {

    fn update(&mut self, message: Message) {
        match message {
            
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let conteudo = text("Hello, World!").size(25);

        container(conteudo)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}