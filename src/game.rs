use sfml::graphics::RenderWindow;
use sfml::system::Clock;

use crate::game::game_tasks::GameTasks;
use crate::game::input::Input;
use crate::game::renderer::Renderer;
use crate::game::snowman_states::SnowmanStates;

mod input;
mod game_tasks;
mod snowman_states;
mod renderer;

pub struct Game {
    // Todo: Stuct sollte privat sein
    game_solution: i8,
    player_input: Vec<char>,
    snowman_state: SnowmanStates,
    input: Input,
    renderer: Renderer,
    is_stopped: bool,
}

impl Game {
    pub fn game_loop(&mut self, window: &mut RenderWindow) {
        let mut clock = Clock::start(); // Ich hoffe hier ist nicht zu viel falsch
        let mut current_frame = 0;

        // Snowman Control Center
        let _snowman_amplifier: f32 = 1.0;


        loop {
            while let Some(event) = window.poll_event() {
                match self.input.parse_input(event).unwrap() {
                    GameTasks::Close => self.is_stopped = true,
                    //GameTasks::ClickPressed(button, x, y) => ,
                    //GameTasks::ClickReleased(
                    //GameTasks::MouseWheelScrolled(wheel, delta, x, y) => ,
                    GameTasks::Typed(key) => !todo!(),
                    _ => {}
                }
            }
            self.renderer.render(window, &self.player_input, current_frame, self.snowman_state);

            // some things to get an fps counter
            let elapsed_time = clock.elapsed_time().as_milliseconds();
            let mut fps = 0;
            if elapsed_time != 0 { // to remove a error, in which a number gets divided by zero
                fps = 1000_i32 / elapsed_time;
            }
            window.set_title(&format!("Frametime: {}, FPS: {}", clock.elapsed_time().as_milliseconds(), fps));
            clock.restart();
            if self.is_stopped { break; }
            current_frame += 1;
        }
    }
}

pub fn new(number: i8) -> Game {
    Game {
        game_solution: number,
        player_input: Vec::with_capacity(8), // keine Lösung wurde angegeben
        snowman_state: SnowmanStates::Idle,
        input: input::new(),
        renderer: renderer::new(),
        is_stopped: false,
    }
}
