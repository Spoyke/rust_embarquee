#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bargraph::Bargraph;
use tp_rust_embarquee::bsp::Board;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();
    let mut value: i32 = 0;
    let min: i32 = 0;
    let max: i32 = 100;
    let step: i32 = 10;

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(min, max);
    bargraph.set_value(value);

    loop {
        value = (value + step) % (max + step);
        bargraph.set_value(value);
        Timer::after_millis(500).await;
        info!("Bargraph's value : {}", value);
    }
}
