use std::time::Duration;

use cosmic::iced::{Rectangle, Size, Subscription, event::listen_with};

use crate::{
    config::{APP_ID, Flags, WeatherConfig, flags},
    fl,
    weather::{IpApi, WeatherApi},
};

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<Weather>(flags())
}

#[derive(Debug, Clone, Default)]
pub struct WeatherUpdate {
    pub temp: i32,
    pub icon: String,
    pub uv: f32,
    pub city: Option<String>,
    pub region: Option<String>,
}

struct Weather {
    core: cosmic::app::Core,
    popup: Option<cosmic::iced::window::Id>,
    config: WeatherConfig,
    config_handler: Option<cosmic::cosmic_config::Config>,
    temperature: i32,
    icon: String,
    uv: f32,
    latitude: String,
    longitude: String,
    city: String,
    region: String,
    use_fahrenheit: bool,
    use_ip_location: bool,
    size: Size,
}

impl Weather {
    fn update_weather_data(&mut self) -> cosmic::app::Task<Message> {
        if self.use_ip_location {
            cosmic::Task::perform(
                async {
                    let ip = IpApi::get_location_from_ip()
                        .await
                        .map_err(|e| format!("IP Location API Error: {}", e))?;
                    let weather =
                        WeatherApi::get_location_forecast(ip.lat.to_string(), ip.lon.to_string())
                            .await
                            .map_err(|e| format!("Forecast API Error: {}", e))?;

                    Ok(WeatherUpdate {
                        temp: weather.temp,
                        icon: weather.icon,
                        uv: weather.uv,
                        city: Some(ip.city),
                        region: Some(ip.regionName),
                    })
                },
                |result: Result<WeatherUpdate, String>| match result {
                    Ok(update) => cosmic::action::Action::App(Message::UpdateApplet(update)),
                    Err(e) => {
                        tracing::error!("{}", e);
                        cosmic::action::Action::App(Message::UpdateApplet(WeatherUpdate {
                            icon: String::from("weather-clear"),
                            ..Default::default()
                        }))
                    }
                },
            )
        } else {
            cosmic::Task::perform(
                WeatherApi::get_location_forecast(
                    self.config.latitude.to_string(),
                    self.config.longitude.to_string(),
                ),
                |result| match result {
                    Ok(weather) => {
                        cosmic::action::Action::App(Message::UpdateApplet(WeatherUpdate {
                            temp: weather.temp,
                            icon: weather.icon,
                            uv: weather.uv,
                            ..Default::default()
                        }))
                    }
                    Err(e) => {
                        tracing::error!("Failed to get location forecast: {e:?}");
                        cosmic::action::Action::App(Message::UpdateApplet(WeatherUpdate {
                            icon: String::from("weather-clear"),
                            ..Default::default()
                        }))
                    }
                },
            )
        }
    }

    fn format_temperature(&self) -> String {
        if self.use_fahrenheit {
            let fahrenheit = (self.temperature as f64 * 9.0 / 5.0 + 32.0).round() as i32;
            format!("{fahrenheit}°F")
        } else {
            format!("{}°C", self.temperature)
        }
    }

    fn location_display(&self) -> Option<String> {
        if self.city.is_empty() && self.region.is_empty() {
            None
        } else {
            Some(format!("{}, {}", self.city, self.region))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Size(Size),
    Tick,
    ToggleWindow,
    PopupClosed(cosmic::iced::window::Id),
    UpdateApplet(WeatherUpdate),
    UpdateLatitude(String),
    UpdateLongitude(String),
    ToggleFahrenheit(bool),
    ToggleIpLocation(bool),
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
        let config = flags.config;

        (
            Self {
                core,
                popup: None,
                latitude: format!("{:.4}", config.latitude),
                longitude: format!("{:.4}", config.longitude),
                use_fahrenheit: config.use_fahrenheit,
                use_ip_location: config.use_ip_location,
                config,
                config_handler: flags.config_handler,
                temperature: 0,
                icon: String::from("weather-clear"),
                uv: 0.0,
                city: String::new(),
                region: String::new(),
                size: Size {
                    width: 10.,
                    height: 10.,
                },
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
        Subscription::batch([
            listen_with(|event, _status, id| {
                if let cosmic::iced::Event::Window(
                    cosmic::iced::window::Event::Resized(size)
                    | cosmic::iced::window::Event::Opened { position: _, size },
                ) = event
                    && id == cosmic::iced::window::Id::RESERVED
                {
                    Some(Message::Size(size))
                } else {
                    None
                }
            }),
            cosmic::iced::time::every(Duration::from_secs(60)).map(|_| Message::Tick),
        ])
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }

    fn on_close_requested(&self, id: cosmic::iced::window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::Tick => {
                return self.update_weather_data();
            }
            Message::UpdateApplet(update) => {
                self.icon = update.icon;
                self.temperature = update.temp;
                self.uv = update.uv;

                if let Some(city) = update.city {
                    self.city = city;
                }
                if let Some(region) = update.region {
                    self.region = region;
                }
            }
            Message::ToggleWindow => {
                if let Some(id) = self.popup.take() {
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
                }

                let new_id = cosmic::iced::window::Id::unique();
                self.popup.replace(new_id);

                let mut popup_settings = self.core.applet.get_popup_settings(
                    self.core.main_window_id().unwrap(),
                    new_id,
                    None,
                    None,
                    None,
                );
                popup_settings.positioner.anchor_rect = Rectangle::<i32> {
                    x: 0,
                    y: 0,
                    width: self.size.width as i32,
                    height: self.size.height as i32,
                };

                return cosmic::iced::platform_specific::shell::commands::popup::get_popup(
                    popup_settings,
                );
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::UpdateLatitude(value) => {
                self.latitude = value.clone();

                if let Some(handler) = &self.config_handler
                    && let Err(e) = self
                        .config
                        .set_latitude(handler, value.parse::<f64>().unwrap_or_default())
                {
                    tracing::error!("{e}");
                }

                return self.update_weather_data();
            }
            Message::UpdateLongitude(value) => {
                self.longitude = value.clone();

                if let Some(handler) = &self.config_handler
                    && let Err(e) = self
                        .config
                        .set_longitude(handler, value.parse::<f64>().unwrap_or_default())
                {
                    tracing::error!("{e}");
                }

                return self.update_weather_data();
            }
            Message::ToggleFahrenheit(value) => {
                self.use_fahrenheit = value;

                if let Some(handler) = &self.config_handler
                    && let Err(e) = self.config.set_use_fahrenheit(handler, value)
                {
                    tracing::error!("{e}");
                }
            }
            Message::ToggleIpLocation(value) => {
                self.use_ip_location = value;

                if let Some(handler) = &self.config_handler
                    && let Err(e) = self.config.set_use_ip_location(handler, value)
                {
                    tracing::error!("{e}");
                }

                return self.update_weather_data();
            }
            Message::Size(size) => {
                self.size = size;
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
                cosmic::widget::row::with_capacity(2)
                    .push(icon)
                    .push(temp)
                    .align_y(cosmic::iced::alignment::Vertical::Center)
                    .spacing(4),
            )
        } else {
            cosmic::Element::from(
                cosmic::iced::widget::column::with_capacity(2)
                    .push(icon)
                    .push(temp)
                    .align_x(cosmic::iced::alignment::Horizontal::Center)
                    .spacing(4),
            )
        };

        let button = cosmic::widget::button::custom(data)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press_down(Message::ToggleWindow);

        cosmic::widget::autosize::autosize(button, cosmic::widget::Id::unique()).into()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> cosmic::Element<'_, Message> {
        let mut data = cosmic::iced::widget::column::with_capacity(10).padding([16, 0]);

        // Weather header with icon, temperature, location, and UV index
        let weather_icon = cosmic::widget::icon::from_name(self.icon.clone())
            .size(48)
            .symbolic(true);

        let mut weather_info = cosmic::iced::widget::column::with_capacity(3)
            .push(cosmic::widget::text::title3(self.format_temperature()))
            .spacing(4);

        if self.use_ip_location
            && let Some(location) = self.location_display()
        {
            weather_info = weather_info.push(
                cosmic::widget::row::with_capacity(2)
                    .push(
                        cosmic::widget::icon::from_name("mark-location-symbolic")
                            .size(14)
                            .symbolic(true),
                    )
                    .push(cosmic::widget::text::body(location))
                    .spacing(4)
                    .align_y(cosmic::iced::alignment::Vertical::Center),
            );
        }

        if self.uv > 0.0 {
            // Colour-code the UV index: white (low), yellow (moderate), red (high).
            let uv_colour = if self.uv < 3.0 {
                cosmic::iced::Color::WHITE
            } else if self.uv < 6.0 {
                cosmic::iced::Color::from_rgb(1.0, 0.85, 0.0)
            } else {
                cosmic::iced::Color::from_rgb(0.9, 0.1, 0.1)
            };

            weather_info = weather_info.push(
                cosmic::widget::row::with_capacity(2)
                    .push(cosmic::widget::text::caption("UV Index: "))
                    .push(
                        cosmic::widget::text::caption(format!("{:.1}", self.uv))
                            .class(cosmic::theme::Text::Color(uv_colour)),
                    ),
            );
        }

        let header = cosmic::widget::row::with_capacity(2)
            .push(weather_icon)
            .push(weather_info)
            .spacing(12)
            .align_y(cosmic::iced::alignment::Vertical::Center);

        data =
            data.push(cosmic::applet::padded_control(header))
                .push(cosmic::applet::padded_control(
                    cosmic::widget::divider::horizontal::default(),
                ));

        // IP location toggle
        let ip_location_toggler = cosmic::widget::row::with_capacity(3)
            .push(cosmic::widget::text(fl!("ip-location-toggle")))
            .push(cosmic::widget::Space::new().width(cosmic::iced::Length::Fill))
            .push(
                cosmic::widget::toggler(self.use_ip_location).on_toggle(Message::ToggleIpLocation),
            );

        data = data.push(cosmic::applet::padded_control(ip_location_toggler));

        // Manual coordinates input (only when not using IP location)
        if !self.use_ip_location {
            data = data.push(cosmic::applet::padded_control(
                cosmic::widget::divider::horizontal::default(),
            ));

            let latitude_col = cosmic::iced::widget::column::with_capacity(2)
                .push(cosmic::widget::text::body(fl!("latitude")))
                .push(
                    cosmic::widget::text_input(fl!("latitude"), &self.latitude)
                        .on_input(Message::UpdateLatitude)
                        .width(cosmic::iced::Length::Fill),
                )
                .spacing(4);

            let longitude_col = cosmic::iced::widget::column::with_capacity(2)
                .push(cosmic::widget::text::body(fl!("longitude")))
                .push(
                    cosmic::widget::text_input(fl!("longitude"), &self.longitude)
                        .on_input(Message::UpdateLongitude)
                        .width(cosmic::iced::Length::Fill),
                )
                .spacing(4);

            let location_row = cosmic::widget::row::with_capacity(2)
                .push(latitude_col)
                .push(longitude_col)
                .spacing(8);

            data = data.push(cosmic::applet::padded_control(location_row));
        }

        data = data.push(cosmic::applet::padded_control(
            cosmic::widget::divider::horizontal::default(),
        ));

        // Temperature unit toggle
        let celsius_btn = cosmic::widget::button::text("°C")
            .class(if self.use_fahrenheit {
                cosmic::theme::Button::Standard
            } else {
                cosmic::theme::Button::Suggested
            })
            .on_press(Message::ToggleFahrenheit(false));

        let fahrenheit_btn = cosmic::widget::button::text("°F")
            .class(if self.use_fahrenheit {
                cosmic::theme::Button::Suggested
            } else {
                cosmic::theme::Button::Standard
            })
            .on_press(Message::ToggleFahrenheit(true));

        let temperature_row = cosmic::widget::row::with_capacity(4)
            .push(cosmic::widget::text(fl!("temperature")))
            .push(cosmic::widget::Space::new().width(cosmic::iced::Length::Fill))
            .push(celsius_btn)
            .push(fahrenheit_btn)
            .spacing(4)
            .align_y(cosmic::iced::alignment::Vertical::Center);

        data = data.push(cosmic::applet::padded_control(temperature_row));

        self.core
            .applet
            .popup_container(cosmic::widget::container(data))
            .into()
    }
}
