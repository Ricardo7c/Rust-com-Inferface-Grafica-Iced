use iced::widget::{column, container, text, text_input};
use iced::{Element, Fill};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .run()
}

#[derive(Default)]
struct App {
    // Estados aqui
    input: String
}


#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    TxtInput(String)
}

impl App {
    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TxtInput(texto) => {
                    self.input = texto;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let texto = if self.input.is_empty(){
            "(nada ainda)"
        }else{
            &self.input
        };


        let content = column![
            text(format!("Você digitou: {}", texto)).size(25),
            text_input("Digite algo...", &self.input).on_input(Message::TxtInput)
        ].spacing(20).align_x(iced::Center).width(500);

        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

}