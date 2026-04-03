use embassy_stm32::gpio::{Level, Output, Speed};

pub struct BargraphPins {
    pub led0: Output<'static>,
    pub led1: Output<'static>,
    pub led2: Output<'static>,
    pub led3: Output<'static>,
    pub led4: Output<'static>,
    pub led5: Output<'static>,
    pub led6: Output<'static>,
    pub led7: Output<'static>,
}

pub struct Board {
    pub bargraph_pins: BargraphPins,
}

impl Board {
    pub fn new() -> Self {
        let p = embassy_stm32::init(Default::default());

        Self {
            bargraph_pins: BargraphPins {
                led0: Output::new(p.PC7, Level::Low, Speed::Low),
                led1: Output::new(p.PB2, Level::Low, Speed::Low),
                led2: Output::new(p.PA8, Level::Low, Speed::Low),
                led3: Output::new(p.PB1, Level::Low, Speed::Low),
                led4: Output::new(p.PB15, Level::Low, Speed::Low),
                led5: Output::new(p.PB4, Level::Low, Speed::Low),
                led6: Output::new(p.PB14, Level::Low, Speed::Low),
                led7: Output::new(p.PB5, Level::Low, Speed::Low),
            },
        }
    }
}
