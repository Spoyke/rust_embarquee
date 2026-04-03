#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bsp::Board;
use tp_rust_embarquee::encoder::Encoder;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();
    let mut encoder = Encoder::new(board.encoder_pins);

    info!("Encoder value : {}", encoder.read_value());
    encoder.set_position(1000);
    info!("Encoder value : {}", encoder.read_value());

    Timer::after_millis(1000).await;

    info!("Encoder value : {}", encoder.read_value());
    encoder.reset();
    info!("Encoder value : {}", encoder.read_value());

    loop {
        info!(
            "Encoder button : {}",
            match encoder.is_pressed() {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        info!("Encoder value : {}", encoder.read_value());

        Timer::after_millis(100).await;
    }
}
