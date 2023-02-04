use sfml::window::Event;

use crate::game::game_tasks::GameTasks;
use crate::game::game_tasks::GameTasks::Typed;

pub struct Input {}

impl Input {
    pub fn parse_input(&self, event: Event) -> Option<GameTasks> {
        /*
        Registriere alle Events, die auf die das Spiel zukommen in einem Enum Array,
        welcher dann an game.rs zurückgegeben wird, damit dieser dann abgearbeitet werden kann
        Vorteil: weniger komplexität
        (aktueller) Nachteil: Länge der Liste ist begrenzt
         */
        match event {
            Event::Closed => Some(GameTasks::Close),
            Event::MouseWheelScrolled { wheel, delta, x, y } => Some(GameTasks::MouseWheelScrolled(wheel, delta, x, y)),
            Event::MouseButtonPressed { button, x, y } => Some(GameTasks::ClickPressed(button, x, y)),
            Event::MouseButtonReleased { button, x, y } => Some(GameTasks::ClickReleased(button, x, y)),
            Event::KeyPressed { code, alt, ctrl, shift, system } =>
                Some(Typed(code)),
            _ => { Some(GameTasks::NOP) } //TODO: Fixen -> könnte schwer lös- und findbare Probleme bringen
        }
    }
}

pub fn new() -> Input {
    Input {}
}
