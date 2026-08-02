use crate::{draw::draw_version, pnp, pnp::Button, utils::CircularCounter};
use crate::pnp::elapsed_time;

use crate::draw::{GREEN, ORANGE, YELLOW};

pub struct MenuOption<Value> {
    label: &'static str,
    value: Value,
}

impl<Value> MenuOption<Value> {
    pub const fn new(value: Value, label: &'static str) -> Self {
        Self { value, label }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCounter0 {
    init_frame5: u32,
    init_frame4: u32,
    init_frame3: u32,
    init_frame2: u32,
    init_frame1: u32,
    init_frame: u32,
    hit_frame: u32,
    orif2: u32,
    orif: u32,
    rst: u32,
    pb0: bool,
}


pub struct Menu<Value: 'static + Copy> {
    is_count: FrameCounter0,
    is_cached: bool,
    is_locked: bool,
    counter: CircularCounter,
    options: &'static [MenuOption<Value>],
}

impl<Value: Copy> Menu<Value> {
    pub fn new(options: &'static [MenuOption<Value>]) -> Self {
        Self {
            is_count: FrameCounter0 { init_frame: (0), init_frame1: (0), init_frame2: (0), init_frame3: (0), init_frame4: (0), init_frame5: (0), hit_frame: (0), orif: (0), orif2: (0), rst: (0), pb0: true },
            is_cached: false,
            is_locked: false,
            counter: CircularCounter::new(1, options.len()),
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

            (_, _) if pnp::is_just_pressed(Button::A) => current_view,//self.value(), // Button::A

            (_, _) => current_view,
        }
    }

    

    fn cursor_str(&self, index: usize) -> &str {
        if self.counter.value() == index {
            "}"
        } else {
            " "
        }
    }

    pub fn draw(&self) {
        for (index, option) in self.options.iter().enumerate() {
            if elapsed_time() / 500 % 5 == 0 {
            pnp::println!(color = YELLOW, "{} {}", self.cursor_str(index + 1), option.label);
            } else if elapsed_time() / 500 % 2 == 0 {
            pnp::println!(color = GREEN, "{} {}", self.cursor_str(index + 1), option.label);
            } else {
            pnp::println!(color = ORANGE, "{} {}", self.cursor_str(index + 1), option.label);
            }        
        }
        pnp::println!("");
        draw_version();
    }


    pub fn update_view(&mut self) {
        if self.is_locked {
            return;
        }

        if self.is_cached {
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

    pub fn update_cached(&mut self) -> bool {
        if pnp::is_just_pressed(Button::B | Button::X) {
            self.is_cached = !self.is_cached;
        }
        self.is_cached
    }
/*
    pub fn update_count(&mut self) -> FrameCounter0 {
        if pnp::is_just_pressed(Button::A) {
            self.is_count.init_frame = 112;
            self.is_count.hit_frame = 113;
            self.is_count.rst = 114;
        }
        self.is_count
    }
*/
}
