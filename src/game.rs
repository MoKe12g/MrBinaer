use sfml::graphics::RenderWindow;
use sfml::system::{Clock, Vector2f};
use sfml::window::Key;

use crate::game::game_tasks::GameTasks;
use crate::game::input::Input;
use crate::game::renderer::Renderer;
use crate::game::snowman_states::SnowmanStates;

mod input;
mod game_tasks;
mod snowman_states;
mod renderer;
mod point_hoap;

pub struct Game {
    origin: Vec<char>,
    game_solution_binary: Vec<u8>,
    player_input: Vec<u8>,
    snowman_state: SnowmanStates,
    snowman_animation_duration: i32,
    input: Input,
    renderer: Renderer,
    is_stopped: bool,
    is_user_terminated: bool,
}

impl Game {
    pub fn game_loop(&mut self, window: &mut RenderWindow) {
        let origin_string = self.origin.clone().into_iter().collect::<String>();
        println!("The chosen number is {}", origin_string);

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
                        self.snowman_state = SnowmanStates::Idle;
                        // exit game
                        println!("Game ended, terminating...");
                        self.is_stopped = true;
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
                    GameTasks::ClickPressed(_, x, y) => {
                        if self.snowman_state != SnowmanStates::IsFirTree() {
                            self.snowman_state = SnowmanStates::DeformationToAvoidPoint(x, y, current_frame)
                        } else {
                            self.snowman_state = SnowmanStates::MorphingFromAFirTree(current_frame);
                        }
                    },
                    GameTasks::ClickReleased(_, x, y) => self.snowman_state = SnowmanStates::ReverseDeformationToAvoidPoint(x, y, current_frame),
                    //GameTasks::MouseWheelScrolled(wheel, delta, x, y) => , // TODO: let the snowman grow!
                    GameTasks::Typed(key) => {
                        if self.game_solution_binary.len() != self.player_input.len() {
                            // where the game logic is hidden
                            let input: u8 = match key {
                                Key::NUM0 | Key::NUMPAD0 => 0,
                                Key::NUM1 | Key::NUMPAD1 => 1,
                                Key::NUM2 | Key::NUMPAD2 => 2,
                                Key::NUM3 | Key::NUMPAD3 => 3,
                                Key::NUM4 | Key::NUMPAD4 => 4,
                                Key::NUM5 | Key::NUMPAD5 => 5,
                                Key::NUM6 | Key::NUMPAD6 => 6,
                                Key::NUM7 | Key::NUMPAD7 => 7,
                                Key::NUM8 | Key::NUMPAD8 => 8,
                                Key::NUM9 | Key::NUMPAD9 => 9,
                                _ => 255, // for any other key, not bound to a event
                            };

                            if *self.game_solution_binary.get(self.player_input.len()).unwrap() == input && self.snowman_state != SnowmanStates::IsFirTree() {
                                self.player_input.push(input);
                                // TODO: Player guessed right event
                                println!("You guessed right");
                                if self.game_solution_binary.len() == self.player_input.len() {
                                    println!("Game ended, playing win animation");
                                    self.snowman_state = SnowmanStates::MorphingIntoAFirTree(current_frame);
                                    // exiting this FirTree Morph exits the game
                                }
                            } else {
                                // Todo: Player guessed wrong event
                                println!("You guessed wrong");
                            }
                        } else if self.snowman_state == SnowmanStates::IsFirTree() {
                            self.snowman_state = SnowmanStates::MorphingFromAFirTree(current_frame);
                        }
                        match key {
                            Key::R => {
                                self.is_stopped = true;
                            },
                            Key::Q => {
                                self.is_user_terminated = true;
                                self.is_stopped = true;
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }

            // snowman idle generator
            if self.snowman_state == SnowmanStates::Idle {
                /*if rand::random::<i8>() == 0 {
                    self.snowman_state = SnowmanStates::Melting(current_frame);
                }*/
            }
            self.renderer.render(window, &self.player_input, &origin_string, current_frame, self.snowman_state);

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
        return self.is_user_terminated;
    }
}

pub fn new(number: u8, snowman_animation_duration: i32) -> Game {
    // convert int to Vector of u8 holding single bits
    let mut game_solution: Vec<u8> = Vec::<u8>::with_capacity(8);
    for i in 0..8 { // ???? 8 ????
        game_solution.push(number >> i & 1);
    }
    game_solution.reverse(); // destroys your mind but is the only implemented solution

    // convert int to Vector of char
    let mut origin: Vec<char> = Vec::<char>::with_capacity(8);
    let number_as_string = number.to_string();
    for c in number_as_string.chars() {
        origin.push(c);
    }

    Game {
        origin,
        game_solution_binary: game_solution,
        player_input: Vec::with_capacity(8), // keine Lösung wurde angegeben
        snowman_state: SnowmanStates::Idle,
        snowman_animation_duration,
        input: input::new(),
        // TODO: Renderer initialization geht gar nicht
        renderer: renderer::new(Vector2f::new(0.0, 600.0), Vector2f::new(25.0, 25.0), snowman_animation_duration),
        is_stopped: false,
        is_user_terminated: false,
    }
}
