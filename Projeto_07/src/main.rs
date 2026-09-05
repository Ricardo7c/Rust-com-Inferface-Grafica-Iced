
use iced::Alignment::Center;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Theme};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .run()
}

struct App {
    // Estados aqui
    tema: Theme
}

impl Default for App {
    fn default() -> Self {
        Self {
            tema: Theme::Dark,
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    // Messages aqui
    Tema(Theme)
}

impl App {
    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tema(tema) => self.tema = tema,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Olá, Mundo!"),
            row![
                button("Dark").on_press(Message::Tema(Theme::Dark)),
                button("Light").on_press(Message::Tema(Theme::Light)),
                button("Tokyo").on_press(Message::Tema(Theme::TokyoNight)),
                button("CatppuccinFrappe").on_press(Message::Tema(Theme::CatppuccinFrappe)),
            ].spacing(20).align_y(Center),
        ].spacing(20).align_x(Center);

        container(content)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        self.tema.clone()
    }
}