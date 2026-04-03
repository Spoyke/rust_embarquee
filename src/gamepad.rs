use crate::bsp::GamepadPins;

use embassy_stm32::gpio::Input;

pub enum Button {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub center: bool,
}

pub struct Gamepad {
    btn_top: Input<'static>,
    btn_bottom: Input<'static>,
    btn_left: Input<'static>,
    btn_right: Input<'static>,
    btn_center: Input<'static>,
}

impl Gamepad {
    pub fn new(pins: GamepadPins) -> Self {
        Self {
            btn_top: pins.btn_top,
            btn_bottom: pins.btn_bottom,
            btn_left: pins.btn_left,
            btn_right: pins.btn_right,
            btn_center: pins.btn_center,
        }
    }

    /// Renvoie l'état du bouton 'button'
    pub fn is_pressed(&mut self, button: Button) -> bool {
        // Les boutons sont paramétrés comme actif à l'état bas.
        // On regarde donc si leurs états est bas pour savoir s'ils sont
        // actifs
        match button {
            Button::Top => self.btn_top.is_low(),
            Button::Bottom => self.btn_bottom.is_low(),
            Button::Left => self.btn_left.is_low(),
            Button::Right => self.btn_right.is_low(),
            Button::Center => self.btn_center.is_low(),
        }
    }

    /// Renvoie l'état de tous les boutons
    pub fn poll(&mut self) -> GamepadState {
        GamepadState {
            top: self.is_pressed(Button::Top),
            bottom: self.is_pressed(Button::Bottom),
            left: self.is_pressed(Button::Left),
            right: self.is_pressed(Button::Right),
            center: self.is_pressed(Button::Center),
        }
    }
}
