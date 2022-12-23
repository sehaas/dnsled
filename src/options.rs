use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// UDP socket to listen on.
    #[clap(long, short, default_value = "0.0.0.0:53", env = "DNS_BIND")]
    pub bind: SocketAddr,

    /// upstream DNS resolver.
    #[clap(long, short, default_value = "9.9.9.10:53", env = "DNS_UPSTREAM")]
    pub upstream: SocketAddr,

    /// upstream DNS resolver.
    #[clap(long, short, default_value = "100", env = "LED_COUNT")]
    pub leds: u32,

    /// WLED API endpoint.
    #[clap(
        long,
        short,
        default_value = "http://10.0.0.10/json/state",
        env = "WLED_API"
    )]
    pub wled_api: String,
}
