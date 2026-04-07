use embassy_stm32::Peri;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::{PA0, PA1, PA6, TIM2, TIM3};

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

pub struct GamepadPins {
    pub btn_top: Input<'static>,
    pub btn_bottom: Input<'static>,
    pub btn_right: Input<'static>,
    pub btn_left: Input<'static>,
    pub btn_center: Input<'static>,
}

pub struct EncoderPins {
    pub enc_btn: Input<'static>,
    pub enc_ch1: Peri<'static, PA0>,
    pub enc_ch2: Peri<'static, PA1>,
    pub timer: Peri<'static, TIM2>,
}

pub struct StepperPins {
    pub dir: Output<'static>,
    pub ms1: Output<'static>,
    pub ms2: Output<'static>,
    pub enn: Output<'static>,
    pub stp: Peri<'static, PA6>,
    pub timer: Peri<'static, TIM3>,
}

pub struct Board {
    pub bargraph_pins: BargraphPins,
    pub gamepad_pins: GamepadPins,
    pub encoder_pins: EncoderPins,
    pub stepper_pins: StepperPins,
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
            gamepad_pins: GamepadPins {
                btn_top: Input::new(p.PC8, Pull::Up),
                btn_bottom: Input::new(p.PB11, Pull::Up),
                btn_right: Input::new(p.PC9, Pull::Up),
                btn_left: Input::new(p.PC6, Pull::Up),
                btn_center: Input::new(p.PC5, Pull::Up),
            },
            encoder_pins: EncoderPins {
                enc_btn: Input::new(p.PA15, Pull::Up),
                enc_ch1: p.PA0,
                enc_ch2: p.PA1,
                timer: p.TIM2,
            },
            stepper_pins: StepperPins {
                dir: Output::new(p.PA7, Level::Low, Speed::Low),
                ms1: Output::new(p.PA11, Level::Low, Speed::Low),
                ms2: Output::new(p.PB12, Level::Low, Speed::Low),
                enn: Output::new(p.PA12, Level::High, Speed::Low),
                stp: p.PA6,
                timer: p.TIM3,
            },
        }
    }
}
