#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::{bargraph::Bargraph, gamepad::Gamepad};
use tp_rust_embarquee::{bsp::Board, gamepad::GamepadState};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(10, 90);
    bargraph.set_value(90);

    info!("Bargraph allumé");

    let mut gamepad = Gamepad::new(board.gamepad_pins);
    let mut gamepad_state: GamepadState;

    loop {
        gamepad_state = gamepad.poll();

        info!("Gamepad state : ");
        info!("Top      : {}", gamepad_state.top);
        info!("Bottom   : {}", gamepad_state.bottom);
        info!("Left     : {}", gamepad_state.left);
        info!("Right    : {}", gamepad_state.right);
        info!("Center   : {}", gamepad_state.center);
        Timer::after_millis(500).await;
    }
}
