use std::time::Duration;

use chrono::{Local, Timelike};

use crate::{
    config::{APP_ID, Flags, MOON_ICON, SUN_ICON, WeatherConfig, flags},
    weather::get_location_forecast,
};

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Weather>(flags())
}

struct Weather {
    core: cosmic::app::Core,
    popup: Option<cosmic::iced::window::Id>,
    config: WeatherConfig,
    config_handler: Option<cosmic::cosmic_config::Config>,
    temperature: f64,
    latitude: String,
    longitude: String,
    use_fahrenheit: bool,
}

impl Weather {
    fn update_weather_data(&mut self) -> cosmic::app::Task<Message> {
        cosmic::Task::perform(
            get_location_forecast(
                self.config.latitude.to_string(),
                self.config.longitude.to_string(),
            ),
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

    fn format_temperature(&self) -> String {
        if self.use_fahrenheit {
            format!("{:.1}°F", self.temperature * 9.0 / 5.0 + 32.0)
        } else {
            format!("{:.1}°C", self.temperature)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ToggleWindow,
    UpdateTemperature(f64),
    UpdateLatitude(String),
    UpdateLongitude(String),
    ToggleFahrenheit(bool),
}

impl cosmic::Application for Weather {
    type Flags = Flags;
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = APP_ID;

    fn init(
        core: cosmic::app::Core,
        flags: Self::Flags,
    ) -> (Self, cosmic::app::Task<Self::Message>) {
        let latitude = flags.config.latitude;
        let longitude = flags.config.longitude;
        let use_fahrenheit = flags.config.use_fahrenheit;

        (
            Self {
                core,
                popup: Default::default(),
                config: flags.config,
                config_handler: flags.config_handler,
                temperature: 0.0,
                latitude: latitude.to_string(),
                longitude: longitude.to_string(),
                use_fahrenheit,
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
            Message::Tick => tasks.push(self.update_weather_data()),
            Message::ToggleWindow => {
                if let Some(id) = self.popup.take() {
                    tasks.push(
                        cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(id),
                    )
                } else {
                    let new_id = cosmic::iced::window::Id::unique();
                    self.popup.replace(new_id);

                    let popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );

                    tasks.push(
                        cosmic::iced::platform_specific::shell::commands::popup::get_popup(
                            popup_settings,
                        ),
                    )
                }
            }
            Message::UpdateTemperature(value) => self.temperature = value,
            Message::UpdateLatitude(value) => {
                self.latitude = value.to_string();
                tasks.push(self.update_weather_data());

                if let Some(handler) = &self.config_handler {
                    if let Err(error) = self
                        .config
                        .set_latitude(handler, value.parse::<f64>().unwrap_or_default())
                    {
                        tracing::error!("{error}")
                    }
                }
            }
            Message::UpdateLongitude(value) => {
                self.longitude = value.to_string();
                tasks.push(self.update_weather_data());

                if let Some(handler) = &self.config_handler {
                    if let Err(error) = self
                        .config
                        .set_longitude(handler, value.parse::<f64>().unwrap_or_default())
                    {
                        tracing::error!("{error}")
                    }
                }
            }
            Message::ToggleFahrenheit(value) => {
                self.use_fahrenheit = value;

                if let Some(handler) = &self.config_handler {
                    if let Err(error) = self.config.set_use_fahrenheit(handler, value) {
                        tracing::error!("{error}")
                    }
                }
            }
        }

        cosmic::Task::batch(tasks)
    }

    fn view(&self) -> cosmic::Element<Message> {
        let icon_name = match Local::now().hour() {
            6..18 => SUN_ICON,
            _ => MOON_ICON,
        };

        let icon = cosmic::iced_widget::row![
            cosmic::widget::icon::from_name(icon_name)
                .size(14)
                .symbolic(true),
        ]
        .padding([3, 0, 0, 0]);
        let temperature =
            cosmic::iced_widget::row![cosmic::iced_widget::text(self.format_temperature())];

        let data = cosmic::Element::from(cosmic::iced_widget::row![icon, temperature].spacing(4));
        let button = cosmic::widget::button::custom(data)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::ToggleWindow);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> cosmic::Element<Message> {
        let latitude_input = cosmic::widget::text_input("Latitude", &self.latitude)
            .on_input(Message::UpdateLatitude)
            .width(cosmic::iced::Length::Fill);
        let longitude_input = cosmic::widget::text_input("Longitude", &self.longitude)
            .on_input(Message::UpdateLongitude)
            .width(cosmic::iced::Length::Fill);

        let coordinates_input =
            cosmic::iced_widget::row![latitude_input, longitude_input].spacing(4);

        let fahrenheit_toggler = cosmic::iced_widget::row![
            cosmic::widget::text("Switch to fahrenheit?"),
            cosmic::widget::Space::with_width(cosmic::iced::Length::Fill),
            cosmic::widget::toggler(self.use_fahrenheit).on_toggle(Message::ToggleFahrenheit),
        ];

        let data = cosmic::iced_widget::column![
            cosmic::applet::padded_control(coordinates_input),
            cosmic::applet::padded_control(cosmic::widget::divider::horizontal::default()),
            cosmic::applet::padded_control(fahrenheit_toggler)
        ];

        self.core
            .applet
            .popup_container(cosmic::widget::container(data))
            .into()
    }
}
