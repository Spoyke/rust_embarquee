#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bargraph::Bargraph;
use tp_rust_embarquee::bsp::Board;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Board::new();

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(10, 90);
    bargraph.set_value(90);

    info!("Bargraph allumé");

    loop {}
}
