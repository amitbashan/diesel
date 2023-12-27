use chrono::Timelike;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::component::svg;

use super::{WidgetSize, WidgetStates};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct WeatherWidgetState {
    #[serde(skip)]
    settings: bool,
    location: Option<(f64, f64)>,
}

fn parse_weather_code(code: u8) -> &'static str {
    match code {
        0 => "Clear",
        1..=3 => "Partly cloudy",
        45 | 48 => "Foggy",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing rain",
        71 | 73 | 75 => "Snow fall",
        77 => "Snow grains",
        80..=82 => "Rain showers",
        85 | 86 => "Snow showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => unreachable!(),
    }
}

fn weather_code_to_svg(code: u8) -> fn(Scope) -> Element {
    match code {
        0 => svg::Sun,
        1..=3 => svg::Cloud,
        45 | 48 => svg::CloudFog,
        51 | 53 | 55 | 56 | 57 | 61 | 63 | 65 | 66 | 67 | 80..=82 | 85 | 86 => svg::Drizzle,
        71 | 73 | 75 | 77 => svg::Snow,
        95 | 96 | 99 => svg::Thunderstorm,
        _ => unreachable!(),
    }
}

pub fn WeatherWidget<'a>(
    cx: &'a ScopeState,
    widget_size: WidgetSize,
    widget_states: &'a UseSharedState<WidgetStates>,
) -> Element<'a> {
    let state = widget_states.with(|s| s.weather);
    let settings = state.settings;
    let location = state.location;
    let longitude = location.map(|l| l.0).unwrap_or_default();
    let latitude = location.map(|l| l.1).unwrap_or_default();
    let forecast = use_future(cx, (&location,), |_| async move {
        if let Some((lat, lng)) = location {
            let client = open_meteo_rs::Client::default();
            let mut opts = open_meteo_rs::forecast::Options::default();
            opts.location = open_meteo_rs::Location { lat, lng };
            opts.elevation = Some(open_meteo_rs::forecast::Elevation::Nan);
            opts.current_weather = Some(true);
            opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Celsius);
            opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kmh);
            opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Millimeters);
            opts.time_zone = Some(chrono_tz::Europe::Paris.name().into());
            let start_date = chrono::Utc::now()
                .with_timezone(&chrono_tz::Europe::Paris)
                .naive_local()
                .date();
            opts.start_date = Some(start_date);
            opts.end_date = Some(start_date);
            opts.cell_selection = Some(open_meteo_rs::forecast::CellSelection::Nearest);
            opts.hourly.push("temperature_2m".into());
            opts.hourly.push("weather_code".into());
            opts.daily.push("temperature_2m_max".into());
            opts.daily.push("temperature_2m_min".into());
            client.forecast(opts).await.ok()
        } else {
            None
        }
    });
    let forecast = forecast.value();

    if settings {
        render! {
            div {
                class: "relative flex flex-1 h-full rounded-lg text-xl shadow-xl bg-base-100",
                ondblclick: move |_| {
                    widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                },
                form {
                    class: "flex flex-col justify-center m-2 w-full",
                    onsubmit: move |e| {
                        let values = &e.values;
                        let longitude = &values["longitude"][0];
                        let longitude = longitude.parse::<f64>();
                        let latitude = &values["latitude"][0];
                        let latitude = latitude.parse::<f64>();

                        if let (Ok(longitude), Ok(latitude)) = (longitude, latitude) {
                            widget_states.with_mut(|s| s.weather.location = Some((longitude, latitude)));
                        }
                    },
                    input {
                        hidden: true,
                        r#type: "submit",
                    }
                    span {
                        class: "text-sm",
                        "Longitude:",
                    }
                    input {
                        class: "input input-bordered input-xs w-full",
                        name: "longitude",
                        r#type: "text",
                        value: longitude,
                    }
                    div {
                        span {
                            class: "text-sm",
                            "Latitude:",
                        }
                    }
                    input {
                        class: "input input-bordered input-xs w-full",
                        name: "latitude",
                        r#type: "text",
                        value: latitude,
                    }
                }
            }
        }
    } else {
        if location.is_some() {
            let view = forecast.and_then(|forecast| {
                if let Some(forecast) = forecast {
                    const HOURS_FORECAST: usize = 5;
                    let current_weather = forecast.current_weather.as_ref().unwrap();
                    let temperature = current_weather.temperature.unwrap();
                    let weather_code = current_weather.weathercode.unwrap() as u8;
                    let weather_text = parse_weather_code(weather_code);
                    let weather_icon = weather_code_to_svg(weather_code);
                    let daily = &forecast.daily.as_ref().unwrap()[0];
                    let temperature_low =
                        daily.values["temperature_2m_min"].value.as_f64().unwrap();
                    let temperature_high =
                        daily.values["temperature_2m_max"].value.as_f64().unwrap();
                    let hour = chrono::Local::now().time().hour() as usize;
                    let hourly = forecast
                        .hourly
                        .as_ref()
                        .unwrap()
                        .iter()
                        .skip(hour)
                        .take(HOURS_FORECAST)
                        .enumerate()
                        .map(|(offset, forecast)| {
                            let values = &forecast.values;
                            let hour = hour + offset;
                            let weather_code = values["weather_code"].value.as_u64().unwrap();
                            let temperature = values["temperature_2m"].value.as_f64().unwrap();
                            (hour, weather_code, temperature)
                        });
                    let hourly_forecast = hourly.map(|(hour, weather_code, temperature)| {
                        let weather_icon = weather_code_to_svg(weather_code as u8);
                        render! {
                            div {
                                class: "flex flex-col text-center",
                                div {
                                    class: "text-xs font-bold opacity-85",
                                    "{hour}"
                                }
                                div {
                                    class: "m-auto p-0.5",
                                    weather_icon {}
                                }
                                div {
                                    class: "text-xs font-bold",
                                    "{temperature}°"
                                }
                            }
                        }
                    });

                    match widget_size {
                        WidgetSize::Small => render! {
                            div {
                                class: "flex flex-col flex-1 m-1",
                                div {
                                    class: "font-md font-bold",
                                    "{temperature}°"
                                }
                                div {
                                    class: "flex flex-col flex-1 justify-end",
                                    weather_icon {},
                                    div {
                                        class: "text-xs font-bold",
                                        weather_text
                                    }
                                    div {
                                        class: "text-xs",
                                        "L:{temperature_low}° H:{temperature_high}°",
                                    }
                                }
                            }
                        },
                        WidgetSize::Medium => render! {
                            div {
                                class: "grid grid-rows-2 flex-1 m-1",
                                div {
                                    class: "flex flex-1 flex-justify-between",
                                    div {
                                        class: "font-md font-bold m-1",
                                        "{temperature}°"
                                    }
                                    div {
                                        class: "flex flex-col flex-1 text-right",
                                        div {
                                            class: "ml-auto",
                                            weather_icon {},
                                        }
                                        div {
                                            class: "text-xs font-bold",
                                            weather_text
                                        }
                                        div {
                                            class: "text-xs",
                                            "L:{temperature_low}° H:{temperature_high}°",
                                        }
                                    }
                                }
                                div {
                                    class: "grid grid-cols-5 flex-1",
                                    hourly_forecast
                                }
                            }
                        },
                        _ => unreachable!(),
                    }
                } else {
                    render! {
                        div {
                            class: "flex flex-1 justify-center items-center",
                            div {
                                class: "text-xs font-mono",
                                "Failed to retrieve forecast."
                            }
                        }
                    }
                }
            });

            render! {
                div {
                    class: "relative flex flex-1 h-full rounded-lg text-xl shadow-xl bg-base-100",
                    ondblclick: move |_| {
                        widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                    },
                    view,
                }
            }
        } else {
            render! {
                div {
                    class: "relative flex flex-1 h-full justify-center items-center rounded-lg text-xl shadow-xl bg-neutral",
                    ondblclick: move |_| {
                        widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                    },
                    span {
                        class: "text-sm text-neutral-content",
                        "Location not set."
                    }
                }
            }
        }
    }
}
