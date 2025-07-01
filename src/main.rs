mod sensor_data;

use crate::sensor_data::LocalSensorData;
use craft::components::{Context, Event};
use craft::elements::TinyVg;
use craft::events::CraftMessage;
use craft::geometry::Size;
use craft::resource_manager::ResourceIdentifier;
use craft::style::Weight;
use craft::{components::{Component, ComponentSpecification}, elements::{Container, ElementStyles, Text}, palette, style::{AlignItems, Display, FlexDirection, JustifyContent}, Color};
use std::str::FromStr;

#[derive(Default)]
pub struct PurpleAir {
    sensor_data: Option<LocalSensorData>
}

fn temperature_f(temp: u64) -> String {
    format!("{} °F", temp)
}

fn field(label: &str, value: &str) -> Text {
    Text::new(format!("{}: {}", label, value).as_str())
        .font_size(20.0)
}

const GRAY: Color = Color::from_rgb8(154, 154, 160);

fn row() -> Container {
    Container::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Row)
}

fn column() -> Container {
    Container::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
}

fn hardware_on_the_board(hardware_discovered: String) -> Vec<String> {
    if let Some((_hardware_version, hardware)) = hardware_discovered.split_once("+") {
        return hardware.split("+").map(|s| s.to_string()).collect();
    }

    Vec::new()
}

fn aqi_a(sensor_data: &LocalSensorData) -> Container {
    if let Some(p25aqic) = &sensor_data.p25aqic && let Some(pm2_5_aqi) = sensor_data.pm2_5_aqi {
        let border_radius = 5.0;
        column()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .gap(10)
            .border_width("1px", "1px", "1px", "1px")
            .border_radius(border_radius, border_radius, border_radius, border_radius)
            .background(Color::from_str(p25aqic.as_str()).unwrap_or(Color::WHITE))
            .push(Text::new("Ch A PM2.5 AQI"))
            .push(
                Text::new((pm2_5_aqi as u64).to_string().as_str())
                    .font_size(40.0)
                    .font_weight(Weight::BOLD)
            )
            .width("150px")
            .height("150px")   
    } else {
        column()
    }
}

fn aqi_b(sensor_data: &LocalSensorData) -> Container {
    if let Some(p25aqic_b) = sensor_data.p25aqic_b.as_ref() && let Some(pm2_5_aqi_b) = sensor_data.pm2_5_aqi_b {
        let border_radius = 5.0;
        column()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .gap(10)
            .border_width("1px", "1px", "1px", "1px")
            .border_radius(border_radius, border_radius, border_radius, border_radius)
            .background(Color::from_str(p25aqic_b.as_str()).unwrap_or(Color::WHITE))
            .push(Text::new("Ch B PM2.5 AQI"))
            .push(
                Text::new((pm2_5_aqi_b as u64).to_string().as_str())
                    .font_size(40.0)
                    .font_weight(Weight::BOLD)
            )
            .width("150px")
            .height("150px")   
    } else {
        column()
    }
}

fn common_measurements(sensor_data: &LocalSensorData) -> Container {
    let mut common_measurements = row()
        .align_items(AlignItems::Center)
        .gap(25)
        ;

    if let Some(current_temp_f) = sensor_data.current_temp_f {
        let temp = row()
            .align_items(AlignItems::Center)
            .gap(10)
            .push(
                TinyVg::new(ResourceIdentifier::Bytes(include_bytes!("../assets/device_thermostat_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.tvg")))
                    .width("50px")
                    .height("50px")
                    .max_width("50px")
                    .max_height("50px")
                    .color(Color::from_rgb8(255, 183, 77))
            )
            .push(
                Text::new(temperature_f(current_temp_f).as_str())
                    .font_size(21.0)
                    .color(palette::css::CADET_BLUE)
                    .color(Color::from_rgb8(255, 183, 77))
            );
        
        common_measurements.push_in_place(temp.component());   
    }

    if let Some(current_dewpoint_f) = sensor_data.current_dewpoint_f {
        let dew = row()
            .align_items(AlignItems::Center)
            .gap(10)
            .push(
                TinyVg::new(ResourceIdentifier::Bytes(include_bytes!("../assets/dew_point_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.tvg")))
                    .width("50px")
                    .height("50px")
                    .max_width("50px")
                    .max_height("50px")
                    .color(Color::from_rgb8(128, 203, 196))
            )
            .push(
                Text::new(temperature_f(current_dewpoint_f).as_str())
                    .font_size(21.0)
                    .color(palette::css::CADET_BLUE)
                    .color(Color::from_rgb8(128, 203, 196))
            ); 
        
        common_measurements.push_in_place(dew.component());   
    }

    if let Some(current_humidity) = sensor_data.current_humidity {
        let humdity = row()
            .align_items(AlignItems::Center)
            .gap(10)
            .push(
                TinyVg::new(ResourceIdentifier::Bytes(include_bytes!("../assets/humidity_percentage_24dp_E3E3E3_FILL0_wght400_GRAD0_opsz24.tvg")))
                    .width("50px")
                    .height("50px")
                    .max_width("50px")
                    .max_height("50px")
                    .color(Color::from_rgb8(129, 212, 250))
            )
            .push(
                Text::new(format!("{}%", current_humidity).as_str())
                    .font_size(21.0)
                    .color(palette::css::CADET_BLUE)
                    .color(Color::from_rgb8(129, 212, 250))
            );
        
        common_measurements.push_in_place(humdity.component());   
    }
    
    common_measurements
}

impl Component for PurpleAir {
    type GlobalState = ();
    type Props = ();
    type Message = LocalSensorData;

    fn view(context: &mut Context<Self>) -> ComponentSpecification {
        let sensor_data = context.state().sensor_data.as_ref().unwrap();

        let mut device_container = column()
            .gap(20)
            .border_width("2px", "2px", "2px", "2px")
            .border_color(Color::from_rgb8(25, 27, 42))
            .width("100%")
            .height("100%")
            .padding("25px", "25px", "25px", "25px")
            .background(Color::from_rgb8(35, 37, 52));

        let aqi_container = row().gap(25)
            .push(aqi_a(&sensor_data))
            .push(aqi_b(&sensor_data));

        device_container.push_in_place(aqi_container.component());
        device_container.push_in_place(common_measurements(&sensor_data).component());
        
        device_container.push_in_place(field("Firmware Version", sensor_data.version.as_str()).color(GRAY).component());
        device_container.push_in_place(field("Hardware Version", sensor_data.hardware_version.as_str()).color(GRAY).component());

        let all_hardware = &hardware_on_the_board(sensor_data.hardware_discovered.clone()).join(", ");
        device_container.push_in_place(field("Devices", all_hardware).color(GRAY).component());
        
        device_container.component()
    }

    fn update(context: &mut Context<Self>) {
        let url = "http://10.0.0.158/json?live=true";
        if let craft::events::Message::CraftMessage(CraftMessage::Initialized) = *context.message() {
            let json_data = reqwest::blocking::get(url).unwrap();
            let sensor_data: LocalSensorData = serde_json::from_str(json_data.text().unwrap().as_str()).unwrap();
            
            context.state_mut().sensor_data = Some(sensor_data);
            context.event_mut().future(async move {
                let json_data = reqwest::get(url).await.unwrap();
                let sensor_data: LocalSensorData = serde_json::from_str(json_data.text().await.unwrap().as_str()).unwrap();
                Event::async_result(sensor_data)
            });
        }

        if let craft::events::Message::UserMessage(msg) = context.message() && let Some(sensor_data) = msg.downcast_ref::<LocalSensorData>() {
            context.state_mut().sensor_data = Some(sensor_data.clone());
    
            context.event_mut().future(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                let json_data = reqwest::get(url).await.unwrap();
                let sensor_data: LocalSensorData = serde_json::from_str(json_data.text().await.unwrap().as_str()).unwrap();
                Event::async_result(sensor_data)
            });
        }
    }
}


fn main() {

    use craft::CraftOptions;
    craft::craft_main(PurpleAir::component(), (), CraftOptions {
        renderer: Default::default(),
        window_title: "PurpleAir GUI".to_string(),
        window_size: Some(Size::new(1600.0, 900.0)),
    });
}
