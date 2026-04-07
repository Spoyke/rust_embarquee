use crate::bsp::StepperPins;

use cortex_m::prelude::_embedded_hal_Pwm;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::time::hz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};

use core::sync::atomic::{AtomicU32, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

static STEPPER_DIRECTION: AtomicU32 = AtomicU32::new(0);
static STEPPER_SPEED: AtomicU32 = AtomicU32::new(0);
static STEPPER_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub enum MicrosteppingMode {
    Full,    // MS1=Low,  MS2=Low  → 1   pas par step
    Half,    // MS1=High, MS2=Low  → 1/2 pas par step
    Quarter, // MS1=Low,  MS2=High → 1/4 pas par step
    Eighth,  // MS1=High, MS2=High → 1/8 pas par step
}

#[repr(u32)]
pub enum Direction {
    Clockwise = 0,
    CounterClockwise = 1,
}

pub struct Stepper {
    dir: Output<'static>,
    ms1: Output<'static>,
    ms2: Output<'static>,
    enn: Output<'static>,
    pub pwm: SimplePwm<'static, TIM3>,
}

impl Stepper {
    pub fn new(pins: StepperPins) -> Self {
        let ch1 = PwmPin::new(pins.stp, OutputType::PushPull);

        let pwm = SimplePwm::new(
            pins.timer,
            Some(ch1),
            None,
            None,
            None,
            hz(1),
            Default::default(),
        );

        let mut stepper: Self = Self {
            dir: pins.dir,
            ms1: pins.ms1,
            ms2: pins.ms2,
            enn: pins.enn,
            pwm,
        };

        // Par défaut, le moteur ne tourne pas
        stepper.disable();

        stepper
    }

    pub fn enable(&mut self) -> () {
        self.enn.set_low();
    }

    pub fn disable(&mut self) -> () {
        self.enn.set_high();
    }

    pub fn set_speed(&mut self, speed: u32, direction: Direction) -> () {
        match direction {
            Direction::Clockwise => self.dir.set_high(),
            Direction::CounterClockwise => self.dir.set_low(),
        }

        if speed == 0 {
            self.pwm.ch1().disable();
            return;
        }

        self.pwm.set_frequency(hz(speed));

        let max_duty = self.pwm.max_duty_cycle();
        self.pwm
            .ch1()
            .set_duty_cycle_fraction(max_duty / 2, max_duty); // duty cycle de 50%
        self.pwm.ch1().enable();
    }

    pub fn set_microstepping(&mut self, mode: MicrosteppingMode) -> () {
        match mode {
            MicrosteppingMode::Full => {
                self.ms1.set_low();
                self.ms2.set_low();
            }
            MicrosteppingMode::Half => {
                self.ms1.set_high();
                self.ms2.set_low();
            }
            MicrosteppingMode::Quarter => {
                self.ms1.set_low();
                self.ms2.set_high();
            }
            MicrosteppingMode::Eighth => {
                self.ms1.set_high();
                self.ms2.set_high();
            }
        }
    }

    pub fn update_speed(speed: u32, direction: Direction) {
        STEPPER_SPEED.store(speed, Ordering::Relaxed);
        STEPPER_DIRECTION.store(direction as u32, Ordering::Relaxed);
        STEPPER_SIGNAL.signal(());
    }

    pub async fn wait_and_update(&mut self) {
        STEPPER_SIGNAL.wait().await;
        self.disable();

        let direction = match STEPPER_DIRECTION.load(Ordering::Relaxed) {
            1 => Direction::CounterClockwise,
            _ => Direction::Clockwise,
        };

        let speed = STEPPER_SPEED.load(Ordering::Relaxed);

        self.set_speed(speed, direction);

        self.enable();

        STEPPER_SIGNAL.reset();
    }
}
