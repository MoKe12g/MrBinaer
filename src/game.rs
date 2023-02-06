use sfml::graphics::RenderWindow;
use sfml::system::{Clock, Vector2f};

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
    snowman_animation_duration: i32,
    input: Input,
    renderer: Renderer,
    is_stopped: bool,
}

impl Game {
    pub fn game_loop(&mut self, window: &mut RenderWindow) {
        let mut clock = Clock::start(); // Ich hoffe hier ist nicht zu viel falsch
        let mut current_frame = 0;


        loop {
            // free snowman from his state, if it has ended
            match self.snowman_state {
                SnowmanStates::Waving(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::Jumping(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::TakingTopHat(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::HoldingTopHat()
                    }
                }
                SnowmanStates::PutTopHatBackOn(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::Melting(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Melted
                    }
                }
                SnowmanStates::ResurrectionInProgress(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::Shrinking(_, start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::Growing(amplifire, start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Big(amplifire)
                    }
                }
                SnowmanStates::MorphingIntoAFirTree(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::IsFirTree()
                    }
                }
                SnowmanStates::MorphingFromAFirTree(start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                SnowmanStates::DeformationToAvoidPoint(px, py, start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::IsDeformedToAvoidPoint(px, py);
                    }
                }
                SnowmanStates::ReverseDeformationToAvoidPoint(_, _, start_frame) => {
                    if current_frame - start_frame >= self.snowman_animation_duration {
                        self.snowman_state = SnowmanStates::Idle
                    }
                }
                _ => {}
            }

            while let Some(event) = window.poll_event() {
                match self.input.parse_input(event).unwrap() {
                    GameTasks::Close => self.is_stopped = true,
                    GameTasks::ClickPressed(_, x, y) => self.snowman_state = SnowmanStates::DeformationToAvoidPoint(x, y, current_frame),
                    GameTasks::ClickReleased(_, x, y) => self.snowman_state = SnowmanStates::ReverseDeformationToAvoidPoint(x, y, current_frame),
                    //GameTasks::MouseWheelScrolled(wheel, delta, x, y) => , // TODO: let the snowman grow!
                    GameTasks::Typed(key) => !todo!(),
                    _ => {}
                }
            }

            // snowman idle generator
            if self.snowman_state == SnowmanStates::Idle {
                if rand::random::<i8>() == 0 {
                    self.snowman_state = SnowmanStates::Melting(current_frame);
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

    pub fn got_closed_by_user(&self) -> bool {
        return self.is_stopped;
    }
}

pub fn new(number: i8, snowman_animation_duration: i32) -> Game {
    Game {
        game_solution: number,
        player_input: Vec::with_capacity(8), // keine Lösung wurde angegeben
        snowman_state: SnowmanStates::Idle,
        snowman_animation_duration,
        input: input::new(),
        // TODO: Renderer initialization geht gar nicht
        renderer: renderer::new(Vector2f::new(0.0, 600.0), Vector2f::new(25.0, 25.0), snowman_animation_duration),
        is_stopped: false,
    }
}
