#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bsp::Board;
use tp_rust_embarquee::stepper::{Direction, MicrosteppingMode, Stepper};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    let mut stepper = Stepper::new(board.stepper_pins);
    stepper.set_microstepping(MicrosteppingMode::Full);
    stepper.set_speed(1000, Direction::Clockwise);
    stepper.enable();
    Timer::after_millis(5000).await;
    stepper.disable();
    Timer::after_millis(1000).await;
    stepper.enable();
    stepper.set_speed(10000, Direction::CounterClockwise);

    Timer::after_millis(1000).await;
    stepper.disable();
    stepper.set_microstepping(MicrosteppingMode::Eighth);
    Timer::after_millis(1000).await;
    stepper.enable();

    info!("Stepper lancé");
    loop {
        //Timer::after_millis(500).await;
    }
}
