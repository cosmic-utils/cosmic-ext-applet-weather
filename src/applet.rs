use std::time::Duration;

use crate::{
    config::{APP_ID, Flags, WeatherConfig, flags},
    fl,
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
    temperature: i32,
    icon: String,
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
                Ok((temp, icon)) => {
                    cosmic::action::Action::App(Message::UpdateApplet((temp, icon)))
                }
                Err(error) => {
                    tracing::error!("Failed to get location forecast: {error:?}");
                    cosmic::action::Action::App(Message::UpdateApplet((
                        0,
                        String::from("weather-clear"),
                    )))
                }
            },
        )
    }

    fn format_temperature(&self) -> String {
        if self.use_fahrenheit {
            format!("{}°F", self.temperature * 9 / 5 + 32)
        } else {
            format!("{}°C", self.temperature)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    ToggleWindow,
    PopupClosed(cosmic::iced::window::Id),
    UpdateApplet((i32, String)),
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
                popup: None,
                config: flags.config,
                config_handler: flags.config_handler,
                temperature: 0,
                icon: String::from("weather-clear"),
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

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn on_close_requested(&self, id: cosmic::iced::window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::UpdateApplet((temp, icon)) => {
                self.temperature = temp;
                self.icon = icon;
            }
            Message::Tick => {
                return self.update_weather_data();
            }
            Message::ToggleWindow => {
                if let Some(id) = self.popup.take() {
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
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

                    return cosmic::iced::platform_specific::shell::commands::popup::get_popup(
                        popup_settings,
                    );
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::UpdateLatitude(value) => {
                self.latitude = value.clone();

                if let Some(handler) = &self.config_handler
                    && let Err(error) = self
                        .config
                        .set_latitude(handler, value.parse::<f64>().unwrap_or_default())
                {
                    tracing::error!("{error}")
                }

                return self.update_weather_data();
            }
            Message::UpdateLongitude(value) => {
                self.longitude = value.clone();

                if let Some(handler) = &self.config_handler
                    && let Err(error) = self
                        .config
                        .set_longitude(handler, value.parse::<f64>().unwrap_or_default())
                {
                    tracing::error!("{error}")
                }

                return self.update_weather_data();
            }
            Message::ToggleFahrenheit(value) => {
                self.use_fahrenheit = value;

                if let Some(handler) = &self.config_handler
                    && let Err(error) = self.config.set_use_fahrenheit(handler, value)
                {
                    tracing::error!("{error}")
                }
            }
        };

        cosmic::Task::none()
    }

    fn view(&self) -> cosmic::Element<'_, Message> {
        let temp = self.core.applet.text(self.format_temperature());
        let icon = cosmic::widget::icon::from_name(self.icon.clone())
            .size(self.core.applet.suggested_size(true).0)
            .symbolic(true);

        let data = if self.core.applet.is_horizontal() {
            cosmic::Element::from(
                cosmic::iced_widget::row![icon, temp]
                    .align_y(cosmic::iced::alignment::Vertical::Center)
                    .spacing(4)
                    .padding([0, self.core.applet.suggested_padding(true).1]),
            )
        } else {
            cosmic::Element::from(
                cosmic::iced_widget::column![icon, temp]
                    .align_x(cosmic::iced::alignment::Horizontal::Center)
                    .spacing(4)
                    .padding([self.core.applet.suggested_padding(true).0, 0]),
            )
        };

        let button = cosmic::widget::button::custom(data)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press_down(Message::ToggleWindow);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> cosmic::Element<'_, Message> {
        let latitude_row = cosmic::iced_widget::column![
            cosmic::widget::text(fl!("latitude")),
            cosmic::widget::text_input(fl!("latitude"), &self.latitude)
                .on_input(Message::UpdateLatitude)
                .width(cosmic::iced::Length::Fill)
        ]
        .spacing(4);
        let longitude_row = cosmic::iced_widget::column![
            cosmic::widget::text(fl!("longitude")),
            cosmic::widget::text_input(fl!("longitude"), &self.longitude)
                .on_input(Message::UpdateLongitude)
                .width(cosmic::iced::Length::Fill)
        ]
        .spacing(4);
        let fahrenheit_toggler = cosmic::iced_widget::row![
            cosmic::widget::text(fl!("fahrenheit-toggle")),
            cosmic::widget::Space::with_width(cosmic::iced::Length::Fill),
            cosmic::widget::toggler(self.use_fahrenheit).on_toggle(Message::ToggleFahrenheit),
        ];

        let data = cosmic::iced_widget::column![
            cosmic::applet::padded_control(latitude_row),
            cosmic::applet::padded_control(longitude_row),
            cosmic::applet::padded_control(cosmic::widget::divider::horizontal::default()),
            cosmic::applet::padded_control(fahrenheit_toggler)
        ]
        .padding([16, 0]);

        self.core
            .applet
            .popup_container(cosmic::widget::container(data))
            .into()
    }
}
