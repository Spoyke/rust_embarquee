#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Input;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use tp_rust_embarquee::bsp::Board;
use tp_rust_embarquee::encoder::Encoder;
use tp_rust_embarquee::gamepad::{Gamepad, GamepadState};
use tp_rust_embarquee::stepper::{Direction, MicrosteppingMode, Stepper};
use tp_rust_embarquee::{bargraph::Bargraph, gamepad};

#[embassy_executor::task]
async fn bargraph_task(mut bargraph: Bargraph) {
    loop {
        bargraph.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn encoder_task(mut encoder: Encoder) {
    loop {
        Timer::after_millis(2000).await;
        let raw = encoder.read_value(); // i16
        let value = raw.unsigned_abs() as u32;

        Bargraph::update_value(value);

        let direction = if raw >= 0 {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        };

        Stepper::update_speed(value, direction);
    }
}

#[embassy_executor::task]
async fn stepper_task(mut stepper: Stepper) {
    loop {
        stepper.wait_and_update().await;
    }
}

#[embassy_executor::task]
async fn emergency_stop_task(mut gamepad: Gamepad) {
    loop {
        if gamepad.is_pressed(gamepad::Button::Center) {
            let tim2 = embassy_stm32::pac::TIM2;
            tim2.cr1().modify(|w| w.set_cen(false)); // Désactive le timer
            tim2.cnt().write_value(0); // Remet le compteur à zéro

            Stepper::update_speed(0, Direction::Clockwise);
        }

        Timer::after_millis(10).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Board::new();

    let mut bargraph = Bargraph::new(board.bargraph_pins);
    bargraph.set_range(0, 100);

    let mut encoder = Encoder::new(board.encoder_pins);
    encoder.set_position(0);

    let mut stepper = Stepper::new(board.stepper_pins);
    stepper.set_microstepping(MicrosteppingMode::Full);
    stepper.enable();

    let mut gamepad = Gamepad::new(board.gamepad_pins);

    spawner.spawn(bargraph_task(bargraph)).unwrap();
    spawner.spawn(encoder_task(encoder)).unwrap();
    spawner.spawn(stepper_task(stepper)).unwrap();
    spawner.spawn(emergency_stop_task(gamepad)).unwrap();

    loop {
        Timer::after_millis(100).await;
    }
}
