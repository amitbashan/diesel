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
            opts.daily.push("temperature_2m_max".into());
            opts.daily.push("temperature_2m_min".into());
            let r = client.forecast(opts).await.ok();
            log::info!("{r:#?}");
            r
        } else {
            None
        }
    });
    let forecast = forecast.value();

    if settings {
        render! {
            div {
                class: "relative flex flex-1 h-full rounded-lg text-xl shadow-xl bg-base-100",
                button {
                    class: "absolute top-1 right-1 btn btn-xs btn-circle",
                    onclick: move |_| {
                        widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                    },
                    svg::Gear {}
                }
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
                    let current_weather = forecast.current_weather.as_ref().unwrap();
                    let temperature = current_weather.temperature.unwrap();
                    let day_or_night = (current_weather.is_day.unwrap() != 0)
                        .then_some(render! { svg::Sun {} })
                        .unwrap_or(render! { svg::Moon {} });
                    let weather_code =
                        parse_weather_code(current_weather.weathercode.unwrap() as u8);
                    let daily = &forecast.daily.as_ref().unwrap()[0];
                    let temperature_low =
                        daily.values["temperature_2m_min"].value.as_f64().unwrap();
                    let temperature_high =
                        daily.values["temperature_2m_max"].value.as_f64().unwrap();

                    render! {
                        div {
                            class: "flex flex-col flex-1",
                            div {
                                class: "font-md font-bold m-1",
                                "{temperature}°"
                            }
                            div {
                                class: "flex flex-col flex-1 justify-end m-1",
                                day_or_night,
                                div {
                                    class: "text-xs font-bold",
                                    weather_code
                                }
                                div {
                                    class: "text-xs",
                                    "L:{temperature_low}° H:{temperature_high}°",
                                }
                            }
                        }
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
                    button {
                        class: "absolute top-1 right-1 btn btn-xs btn-circle",
                        onclick: move |_| {
                            widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                        },
                        svg::Gear {}
                    }
                    view,
                }
            }
        } else {
            render! {
                div {
                    class: "relative flex flex-1 h-full justify-center items-center rounded-lg text-xl shadow-xl bg-neutral",
                    button {
                        class: "absolute top-1 right-1 btn btn-xs btn-circle",
                        onclick: move |_| {
                            widget_states.with_mut(|s| s.weather.settings = !s.weather.settings);
                        },
                        svg::Gear {}
                    }
                    span {
                        class: "text-sm text-neutral-content",
                        "Location not set."
                    }
                }
            }
        }
    }
}
