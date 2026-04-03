#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bsp::Board;
use tp_rust_embarquee::gamepad::{Gamepad, GamepadState};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    let mut gamepad = Gamepad::new(board.gamepad_pins);
    let mut gamepad_state: GamepadState;

    loop {
        gamepad_state = gamepad.poll();

        info!("Gamepad state : ");
        info!(
            "Top : {}",
            match gamepad_state.top {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        info!(
            "Bottom : {}",
            match gamepad_state.bottom {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        info!(
            "Left : {}",
            match gamepad_state.left {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        info!(
            "Right : {}",
            match gamepad_state.right {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        info!(
            "Center : {}",
            match gamepad_state.center {
                true => "Pressed",
                false => "Unpressed",
            }
        );
        Timer::after_millis(100).await;
    }
}
