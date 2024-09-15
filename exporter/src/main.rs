use std::net::{Ipv4Addr, SocketAddrV4};

use consts::metric_names::{DHT11_HUMIDITY, DHT11_TEMPERATURE, DHT11_TEMPERATURE_DESC};
use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use sensors::dht11::DHT11;

mod consts;

fn main() {
    let mut sensor = DHT11::new(17).expect("failed to initialize DHT11 sensor");

    PrometheusBuilder::new()
        .with_http_listener(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 13000))
        .install()
        .expect("failed to install metrics exporter");

    describe_gauge!(DHT11_TEMPERATURE, DHT11_TEMPERATURE_DESC);
    describe_gauge!(DHT11_TEMPERATURE, DHT11_TEMPERATURE_DESC);

    loop {
        match sensor.read() {
            Ok(data) => {
                // println!(
                //     "Temperature: {}°C, Humidity: {}%",
                //     data.temperature, data.humidity
                // );
                gauge!(DHT11_TEMPERATURE).set(data.temperature);
                gauge!(DHT11_HUMIDITY).set(data.humidity);
            }
            Err(e) => {
                eprintln!("error reading from sensor: {}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
