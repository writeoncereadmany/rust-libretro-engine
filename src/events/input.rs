use std::any::Any;
use rust_libretro::types::JoypadState;
use crate::entities::entity::Entities;
use crate::events::dispatcher::Dispatcher;
use crate::events::event::{EventTrait, Events};

pub struct ButtonPressed(pub JoypadState);

impl EventTrait for ButtonPressed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct ButtonReleased(pub JoypadState);

impl EventTrait for ButtonReleased {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}


pub struct InputState(pub JoypadState);

impl EventTrait for InputState {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

const ALL_JOYPAD_BUTTONS: [JoypadState; 16] = [
    JoypadState::UP, JoypadState::DOWN, JoypadState::LEFT, JoypadState::RIGHT,
    JoypadState::A, JoypadState::B, JoypadState::X, JoypadState::Y,
    JoypadState::SELECT, JoypadState::START, JoypadState::L, JoypadState::R,
    JoypadState::L2, JoypadState::R2, JoypadState::L3, JoypadState::R3];

pub fn fire_input_events(current_input: JoypadState, old_input: JoypadState, events: &mut Events) {
    events.fire(InputState(current_input));
    for button in &ALL_JOYPAD_BUTTONS {
        fire_presses_and_releases(*button, current_input, old_input, events);
    }
}

fn fire_presses_and_releases(
    button: JoypadState,
    current_input: JoypadState,
    old_input: JoypadState,
    events: &mut Events,
) {
    if current_input.contains(button) && !old_input.contains(button) {
        events.fire(ButtonPressed(button));
    } else if old_input.contains(button) && !current_input.contains(button) {
        events.fire(ButtonReleased(button))
    }
}