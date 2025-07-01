use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};

fn parse_nonstandard_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // "DateTime": "2025/06/29T22:44:14z",

    let s = String::deserialize(deserializer)?;
    let s = s.trim_end_matches('z');
    let naive = NaiveDateTime::parse_from_str(s, "%Y/%m/%dT%H:%M:%S")
        .map_err(serde::de::Error::custom)?;
    Ok(Utc.from_utc_datetime(&naive))
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    NotConfigured = 0,
    InProgress = 1,
    Success = 2,
    Error = 3,
}

impl Status {
    pub fn from_u8(status_number: u8) -> Option<Status> {
        match status_number {
            0 => Some(Status::NotConfigured),
            1 => Some(Status::InProgress),
            2 => Some(Status::Success),
            3 => Some(Status::Error),
            _ => None
        }
    }
}

pub fn parse_status<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: Deserializer<'de>,
{
    let number = u64::deserialize(deserializer)?;
    Status::from_u8(number as u8)
        .ok_or_else(|| de::Error::custom(format!("Invalid status number: {}", number)))
}

pub fn parse_optional_status<'de, D>(deserializer: D) -> Result<Option<Status>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<u64>::deserialize(deserializer)?;
    match opt {
        Some(number) => Status::from_u8(number as u8)
            .ok_or_else(|| de::Error::custom(format!("Invalid status number: {}", number)))
            .map(Some),
        None => Ok(None),
    }
}


/// https://community.purpleair.com/t/sensor-json-documentation/6917
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalSensorData {
    // --- Sensor Information Fields START ---

    /// The Device ID for the sensor. This is identical to the MAC address, but with leading 0’s omitted in each octet
    #[serde(rename = "SensorId")]
    pub sensor_id: String,

    /// The current time reported by the device. This is provided in UTC and ISO 8601 format
    #[serde(rename = "DateTime", deserialize_with = "parse_nonstandard_datetime")]
    pub date_time: DateTime<Utc>,

    /// This is the name of the PurpleAir-**** WiFI network displayed for device setup
    #[serde(rename = "Geo")]
    pub geo: String,

    /// A call to the ESP function getFreeHeap()
    #[serde(rename = "Mem")]
    pub mem: u64,

    /// A call to the ESP function getHeapFragmentation()
    pub memfrag: u64,

    /// A call to the ESP function getMaxFreeBlockSize()
    pub memfb: u64,

    /// A call to the ESP function getFreeContStack()
    pub memcs: u64,

    /// A count of how many times the /json endpoint has been accessed since the device last powered on
    #[serde(rename = "Id")]
    pub id: u64,

    /// This will roughly, but not exactly, match the latitude set during registration
    pub lat: f64,

    /// This will roughly, but not exactly, match the longitude set during registration
    pub lon: f64,

    #[serde(rename = "loggingrate")]
    /// The rate at which the sensor is reporting data
    pub logging_rate: u64,

    /// Whether the sensor is registered as indoor or outdoor
    pub place: String,

    /// The firmware version in use by the device
    pub version: String,

    /// How long in seconds the sensor has been running for
    pub uptime: u64,

    /// The WiFi signal strength of the connected WiFi network
    pub rssi: i64,

    /// The number of seconds the last 2-minute average recorded data for. This should typically have a value of 120
    pub period: u32,

    #[serde(rename = "httpsuccess")]
    /// The number of successful HTTP requests sent by the device
    pub http_success: u64,

    #[serde(rename = "httpsends")]
    /// The total number of HTTP requests sent by the device
    pub http_sends: u64,

    #[serde(rename = "hardwareversion")]
    /// This indicates the version of the device board. The first PurpleAir devices are 1.0. The PurpleAir Classic and PA-I Indoor are 2.0. The PurpleAir Touch, Flex, and Zen are 3.0
    pub hardware_version: String,

    #[serde(rename = "hardwarediscovered")]
    /// A label indicating all hardware the device board can see. This includes things such as the laser counters and BME environmental sensor
    pub hardware_discovered: String,

    #[serde(rename = "wlstate")]
    /// Whether the device is connected to a WiFi connection
    pub wl_state: String,

    /// The SSID of the connected WiFi network
    pub ssid: String,

    /// This indicates the HTTP response code of sending data to the data processor.
    /// This field is only present if a Data Processor was set for your sensor during registration.
    /// Currently, Weather Underground is set as the first data processor by default.
    pub response: Option<String>,

    /// This indicates the time the sensor last received a response when sending data to a data processor.
    /// This field is only present if a Data Processor was set for your sensor during registration.
    /// Currently, Weather Underground is set as the first data processor by default.
    pub response_date: Option<String>,

    // --- Sensor Information Fields END ---



    // --- Sensor Data Fields START ---

    /// The analog voltage on ADC input of the PurpleAir sensor control board
    /// Required Hardware: -
    #[serde(rename = "Adc")]
    pub adc: f64,

    /// The temperature measured in Fahrenheit. This is uncorrected
    /// Required Hardware: BME
    pub current_temp_f: Option<u64>,

    /// The relative humidity as measured by the device. This is uncorrected
    /// Required Hardware: BME
    pub current_humidity: Option<u64>,

    /// The dewpoint as measured by the device
    /// Required Hardware: BME
    pub current_dewpoint_f: Option<u64>,
    
    /// The barometric pressure measured in millibar
    /// Required Hardware: BME
    pub pressure: Option<f64>,

    /// The temperature as measured by the BME680 if the device has one. This is the same as current_temp_f, unless the device has both a BME280 and a BME680/BME688
    /// Required Hardware: BME68X
    pub current_temp_f_680: Option<f64>,

    /// The humidity as measured by the BME680 if the device has one. This is the same as current_humidity, unless the device has both a BME280 and a BME680/BME688
    /// Required Hardware: BME68X
    pub current_humidity_680: Option<f64>,

    /// The dewpoint as measured by the BME680 if the device has one. This is the same as current_dewpoint_f, unless the device has both a BME280 and a BME680/BME688
    /// Required Hardware: BME68X
    pub current_dewpoint_f_680: Option<f64>,

    /// The barometric pressure as measured by the BME680 if the device has one. This is the same as pressure, unless the device has both a BME280 and a BME680/BME688
    /// Required Hardware: BME68X
    pub pressure_680: Option<f64>,

    /// VOC values read by the BME sensor. NaN means that there is no reading. These readings are still experimental
    /// Required Hardware: BME68X
    pub gas_680: Option<f64>,

    /// The RGB value for AQI LEDs. This is based on the US EPA PM2.5 AQI readings from channel B
    /// Required Hardware: PMSX003-B
    pub p25aqic_b: Option<String>,

    /// US EPA PM2.5 AQI as measured by channel B
    /// Required Hardware: PMSX003-B
    #[serde(rename = "pm2.5_aqi_b")]
    pub pm2_5_aqi_b: Option<f64>,

    /// PM1 readings from channel B using the CF=1 estimation of density
    /// Required Hardware: PMSX003-B
    pub pm1_0_cf_1_b: Option<f64>,

    /// Channel B 0.3-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_0_3_um_b: Option<f64>,

    /// PM2.5 readings from channel B using the CF=1 estimation of density
    /// Required Hardware: PMSX003-B
    pub pm2_5_cf_1_b: Option<f64>,

    /// Channel B 0.5-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_0_5_um_b: Option<f64>,

    /// PM10 readings from channel B using the CF=1 estimation of density
    /// Required Hardware: PMSX003-B
    pub pm10_0_cf_1_b: Option<f64>,

    /// Channel B 1.0-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_1_0_um_b: Option<f64>,

    /// PM1 readings from channel B using the ATM estimation of density
    /// Required Hardware: PMSX003-B
    pub pm1_0_atm_b: Option<f64>,

    /// Channel B 2.5-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_2_5_um_b: Option<f64>,

    /// PM2.5 readings from channel B using the ATM estimation of density
    /// Required Hardware: PMSX003-B
    pub pm2_5_atm_b: Option<f64>,

    /// Channel B 5.0-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_5_0_um_b: Option<f64>,

    /// PM10 readings from channel B using the ATM estimation of density
    /// Required Hardware: PMSX003-B
    pub pm10_0_atm_b: Option<f64>,

    /// Channel B 10.0-micrometer particle counts per deciliter of air
    /// Required Hardware: PMSX003-B
    pub p_10_0_um_b: Option<f64>,

    /// The RGB value for AQI LEDs. This is based on the US EPA PM2.5 AQI readings from channel A
    /// Required Hardware: PMSX003-A
    pub p25aqic: Option<String>,

    #[serde(rename = "pm2.5_aqi")]
    /// US EPA PM2.5 AQI as measured by channel A
    /// Required Hardware: PMSX003-A
    pub pm2_5_aqi: Option<f64>,

    /// PM1 readings from channel A using the CF=1 estimation of density
    /// Required Hardware: PMSX003-A
    pub pm1_0_cf_1: Option<f64>,

    /// Channel A 0.3-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_0_3_um: Option<f64>,

    /// PM2.5 readings from channel A using the CF=1 estimation of density
    /// Required Hardware: PMSX003-A
    pub pm2_5_cf_1: Option<f64>,

    /// Channel A 0.5-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_0_5_um: Option<f64>,

    /// PM10 readings from channel A using the CF=1 estimation of density
    /// Required Hardware: PMSX003-A
    pub pm10_0_cf_1: Option<f64>,

    /// Channel A 1.0-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_1_0_um: Option<f64>,

    /// PM1 readings from channel A using the ATM estimation of density
    /// Required Hardware: PMSX003-A
    pub pm1_0_atm: Option<f64>,

    /// Channel A 2.5-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_2_5_um: Option<f64>,

    /// PM2.5 readings from channel A using the ATM estimation of density
    /// Required Hardware: PMSX003-A
    pub pm2_5_atm: Option<f64>,

    /// Channel A 5.0-micrometer and larger particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_5_0_um: Option<f64>,

    /// PM10 readings from channel A using the ATM estimation of density
    /// Required Hardware: PMSX003-A
    pub pm10_0_atm: Option<f64>,

    /// Channel A 10.0-micrometer particle counts per deciliter of air
    /// Required Hardware: PMSX003-A
    pub p_10_0_um: Option<f64>,

    // --- Sensor Data Fields END ---


    // --- Status Fields START ---

    /// NTP: Network Time Protocol time sync
    #[serde(rename = "status_0", deserialize_with = "parse_status")]
    pub status_ntp: Status,

    /// LOC: Google location lookup
    #[serde(rename = "status_1", deserialize_with = "parse_status")]
    pub status_loc: Status,

    /// UPD: Update check
    #[serde(rename = "status_2", deserialize_with = "parse_status")]
    pub status_upd: Status,

    /// PAA: Connection to PurpleAir servers
    #[serde(rename = "status_3", deserialize_with = "parse_status")]
    pub status_paa: Status,

    /// TSA: ThingSpeak A Channel (no longer used)
    #[serde(rename = "status_4", deserialize_with = "parse_status")]
    pub status_tsa: Status,

    /// TSS: ThingSpeak A Secondary (no longer used)
    #[serde(rename = "status_5", deserialize_with = "parse_status")]
    pub status_tss_a: Status,

    /// 3RD: Status for Data Processor #1 (if setup in the sensor’s registration)
    #[serde(rename = "status_6", deserialize_with = "parse_optional_status", default)]
    pub status_for_processor_1: Option<Status>,

    /// TSB: ThingSpeak B Channel (no longer used)
    #[serde(rename = "status_7", deserialize_with = "parse_status")]
    pub status_tsb: Status,

    /// TSS: ThingSpeak B Secondary (no longer used)
    #[serde(rename = "status_8", deserialize_with = "parse_status")]
    pub status_tss_b: Status,

    /// 3RD: Status for Data Processor #2 (if setup in the sensor’s registration)
    #[serde(rename = "status_10", deserialize_with = "parse_optional_status", default)]
    pub status_for_processor_2: Option<Status>,

    // --- Status Fields END ---
}