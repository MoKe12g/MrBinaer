use sfml::window::Key;
use sfml::window::mouse::{Button, Wheel};

#[derive(Copy, Clone)] // Arbeit an den Compiler weitergeben
pub enum GameTasks {
    NOP,
    // No Operation
    ClickPressed(Button, i32, i32),
    ClickReleased(Button, i32, i32),
    MouseWheelScrolled(Wheel, f32, i32, i32),
    Typed(Key),
    Close,
}