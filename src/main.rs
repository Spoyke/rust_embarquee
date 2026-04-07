#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bargraph::Bargraph;
use tp_rust_embarquee::bsp::Board;
use tp_rust_embarquee::encoder::Encoder;
use tp_rust_embarquee::gamepad::{Gamepad, GamepadState};
use tp_rust_embarquee::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph) {
    loop {
        bargraph.wait_and_update().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Board::new();

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(0, 100);

    spawner.spawn(bargraph_task(bargraph)).unwrap();

    Bargraph::update_value(50);

    loop {
        Timer::after_millis(100).await;
    }
}
