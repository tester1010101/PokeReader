
//
use crate::draw::{BLUE, GREEN, ORANGE, PURPLE, YELLOW};
use crate::{draw::draw_version, pnp, pnp::Button, utils::CircularCounter};
use crate::pnp::elapsed_time;
//

pub trait MenuOptionValue: Copy {
    fn get_label(option: Self) -> &'static str;
}

pub struct MenuOption<Value: MenuOptionValue> {
    label: &'static str,
    value: Value,
}

impl<Value: MenuOptionValue> MenuOption<Value> {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            label: Value::get_label(value),
        }
    }
}

pub struct Menu<const MAX: usize, Value: MenuOptionValue> {
    is_locked: bool,
    counter: CircularCounter<1, MAX>,
    options: [MenuOption<Value>; MAX],
}

impl<const MAX: usize, Value: MenuOptionValue> Menu<MAX, Value> {
    pub fn new(options: [MenuOption<Value>; MAX]) -> Self {
        Self {
            is_locked: false,
            counter: CircularCounter::default(),
            options,
        }
    }

    fn value(&self) -> Value {
        let index = self.counter.value() - 1;
        self.options[index].value
    }

    pub fn next_view(&self, main_menu: Value, current_view: Value) -> Value {
        match (self.is_locked, current_view) {
            (false, _main_menu) if pnp::is_just_pressed(Button::Dright) => self.value(),
            (false, _) if pnp::is_just_pressed(Button::Dleft) => main_menu,
            (_, _) => current_view,
        }
    }

    fn cursor_str(&self, index: usize) -> &str {
        if self.counter.value() == index { "}" } else { " " }
    }

    pub fn draw(&self) {
        //let rng: &RngWrapper<Sfmt>;
        for (index, option) in self.options.iter().enumerate() {
            if elapsed_time() / 1000 % 5 == 0 {
            pnp::println!(color = YELLOW, "{} {}", self.cursor_str(index + 1), option.label);
            } else if elapsed_time() / 1000 % 2 == 0 {
            pnp::println!(color = GREEN, "{} {}", self.cursor_str(index + 1), option.label);
            } else {
            pnp::println!(color = ORANGE, "{} {}", self.cursor_str(index + 1), option.label);
            }
            //pnp::println!(color = RED, "{} {}", self.cursor_str(index + 1), option.label);
        }
        pnp::println!("");//   EOFTAG");
        draw_version();
        //check_frame1();
        //draw_seed(rng);
    }

    pub fn update_view(&mut self) {
        if self.is_locked {
            return;
        }

        if pnp::is_just_pressed(Button::Dup) {
            self.counter.decrement();
        } else if pnp::is_just_pressed(Button::Ddown) {
            self.counter.increment();
        }
    }

    pub fn update_lock(&mut self) -> bool {
        if pnp::is_just_pressed(Button::X | Button::Y) {
            self.is_locked = !self.is_locked;
        }
        self.is_locked
    }
}