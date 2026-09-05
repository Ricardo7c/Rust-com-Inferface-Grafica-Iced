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
    entrada: String,
}

#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    Numero(String)
}

impl App {
    fn title(&self) -> String {
        String::from("Dobrador de Números")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Numero(num) => self.entrada = num
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let texto =  if self.entrada.trim().is_empty() {
                "Digite um número inteiro.".to_string()
            }else{
                match self.entrada.trim().parse::<i32>(){
                    Ok(num) => format!("O dobro de {} é {}", num, num * 2),
                    Err(_) => "Erro: valor digitado não é um número válido!".to_string()
                }
            };

        let content = column![
            text_input("", &self.entrada).on_input(Message::Numero),
            text(texto)
        ].width(400).spacing(20);

        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

}
