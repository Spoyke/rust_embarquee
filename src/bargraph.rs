use crate::bsp::BargraphPins;

use embassy_stm32::gpio::Output;

pub struct Bargraph {
    leds: [Output<'static>; 8],
    min: i32,
    max: i32,
}

impl Bargraph {
    // Par défaut, la valeur de la 'range' est 0-100 pour fonctionner avec un pourcentage
    pub fn new(pins: BargraphPins) -> Self {
        Self {
            leds: [
                pins.led0, pins.led1, pins.led2, pins.led3, pins.led4, pins.led5, pins.led6,
                pins.led7,
            ],
            min: 0,
            max: 100,
        }
    }

    pub fn set_range(&mut self, min: i32, max: i32) -> () {
        self.min = min;
        self.max = max;
    }

    /// Allume les leds du bargraph en fonction de la valeur de 'value' si elle est comprise entre les bornes de la structure
    /// Transforme 'value' en une valeur comprise entre 0 et 1 multiplié par 8 pour obtenir le nombre de led à allumé correspondant à la valeur de value
    pub fn set_value(&mut self, value: i32) -> () {
        if value > self.max || value < self.min {
            return;
        }

        // Comme nb_led_to_activate est un i32, la formule nous donne un entier
        let nb_led_to_activate: i32 =
            ((value - self.min) * self.leds.len() as i32) / (self.max - self.min);

        for (i, led) in self.leds.iter_mut().enumerate() {
            if i < nb_led_to_activate as usize {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }
}
