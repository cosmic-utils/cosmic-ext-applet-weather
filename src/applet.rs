use std::time::Duration;

use crate::{
    config::{APP_ID, SUN_ICON, WeatherConfig},
    weather::get_location_forecast,
};

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Weather>(())
}

struct Weather {
    core: cosmic::app::Core,
    config: WeatherConfig,
    temperature: f64,
}

impl Weather {
    fn update_weather_data(&mut self) -> cosmic::app::Task<Message> {
        cosmic::Task::perform(
            get_location_forecast(self.config.latitude, self.config.longitude),
            |result| match result {
                Ok(temperature) => {
                    cosmic::action::Action::App(Message::UpdateTemperature(temperature))
                }
                Err(error) => {
                    tracing::error!("Failed to get location forecast: {error:?}");
                    cosmic::action::Action::App(Message::UpdateTemperature(0.0))
                }
            },
        )
    }

    fn update_config(&mut self) {
        self.config = WeatherConfig::config();
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    UpdateTemperature(f64),
}

impl cosmic::Application for Weather {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = APP_ID;

    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (Self, cosmic::app::Task<Self::Message>) {
        let config = WeatherConfig::config();

        (
            Self {
                core,
                config,
                temperature: 0.0,
            },
            cosmic::task::message(Message::Tick),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(60)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> cosmic::app::Task<Self::Message> {
        let mut tasks = vec![];

        match message {
            Message::Tick => {
                self.update_config();
                tasks.push(self.update_weather_data());
            }
            Message::UpdateTemperature(temperature) => self.temperature = temperature,
        }

        cosmic::Task::batch(tasks)
    }

    fn view(&self) -> cosmic::Element<Message> {
        let icon = cosmic::iced_widget::row![
            cosmic::widget::icon::from_name(SUN_ICON)
                .size(14)
                .symbolic(true),
        ]
        .padding([3, 0, 0, 0]);
        let temperature =
            cosmic::iced_widget::row![cosmic::iced_widget::text(format!("{}°C", self.temperature))];

        let data = cosmic::Element::from(cosmic::iced_widget::row![icon, temperature].spacing(4));
        let button = cosmic::widget::button::custom(data).class(cosmic::theme::Button::AppletIcon);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }
}
