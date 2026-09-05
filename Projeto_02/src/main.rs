use iced::Alignment::Center;
use iced::widget::{button, container, column, text};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .run()
}

#[derive(Default)]
struct App {
    // Estados aqui
    boleano: bool,
}

#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    Toggle
}

impl App {
    fn title(&self) -> String {
        String::from("Toggle Text")
    }

    fn update(&mut self, message: Message) {
            match message{
                Message::Toggle => {self.boleano = !self.boleano}
            }
    }


    fn view(&self) -> Element<'_, Message> {
        let (texto, botao) = if self.boleano {
            ("Ligado","Desligar")
        } else {
           ("Desligado","Ligar")
        };

        let content = column![
            text(format!("Estado: {}",texto)).size(20),
            button(botao).on_press(Message::Toggle),
        ].spacing(10).width(200).align_x(Center);

        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

}