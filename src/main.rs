use iced::{
    executor,
    Align,
    Application,
    Clipboard,
    Column,
    Command,
    Container,
    Element,
    Image,
    Length,
    Settings,
    Subscription,
    Text,
};
use std::path::PathBuf;

pub fn main() -> iced::Result {
    Events::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Events {
    path: PathBuf,
}


#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
}

impl Application for Events {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (Events::default(), Command::none())
    }

    // window title
    fn title(&self) -> String {
        String::from("Viewer for Image")
    }
    
    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let iced_native::event::Event::Window(we) = event {
                    if let iced_native::window::Event::FileDropped(path) = we {
                        self.path = path;
                    }
                }
            }
        };
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<Message> {
        let mut p = self.path.to_str().unwrap_or("").to_string();
        if p.is_empty() {
            p = String::from("Drop the Image File.");
        }
        let path = Container::new(Text::new(p).size(20)).padding(4);

        let image = Container::new(
            Image::new(self.path.clone())
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center);

        let content = Column::new()
        .width(Length::Fill)
        .align_items(Align::Start)
        .push(path)
        .push(image);

        Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}



