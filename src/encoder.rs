use crate::bsp::EncoderPins;

use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::qei::{Config, Qei, QeiMode};

pub struct Encoder {
    btn: Input<'static>,
    qei: Qei<'static, TIM2>,
}

impl Encoder {
    pub fn new(pins: EncoderPins) -> Self {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.arr().write_value(10_000); // ARR : Auto-Reload Register, valeur maximale du compteur
        tim2.cnt().write_value(5_000); // CNT : Counter Register, définit la valeur actuelle du compteur

        let config = Config {
            ch1_pull: Pull::Up,
            ch2_pull: Pull::Up,
            mode: QeiMode::Mode1,
        };

        let qei = Qei::new(pins.timer, pins.enc_ch1, pins.enc_ch2, config);

        Self {
            btn: pins.enc_btn,
            qei: qei,
        }
    }

    /// Renvoie l'état du bouton de l'encodeur
    pub fn is_pressed(&mut self) -> bool {
        self.btn.is_low()
    }

    /// Renvoie la valeur de l'encodeur
    pub fn read_value(&mut self) -> i16 {
        self.qei.count().wrapping_sub(u16::MIN) as i16
    }

    /// Définit la position (valeur) de l'encodeur
    pub fn set_position(&mut self, position: i32) -> () {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.cnt().write_value(position as u32);
    }

    /// Reinitialise la valeur du timer
    pub fn reset(&mut self) -> () {
        let tim2 = embassy_stm32::pac::TIM2;
        tim2.cnt().write_value(5_000);
    }
}
