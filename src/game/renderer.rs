use std::ops::Deref;

use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable, Vertex, VertexArray};
use sfml::SfBox;
use sfml::system::Vector2f;

use crate::game::snowman_states::SnowmanStates;

pub struct Renderer {
    snowman_pos: Vector2f,
    snowman_scale: Vector2f,
    animation_duration: i32,
    font: SfBox<Font>,
}

impl Renderer {
    pub(crate) fn render(&self, window: &mut RenderWindow, player_input: &Vec<u8>, origin: &String, current_frame: i32, snowman_state: SnowmanStates) {
        window.clear(Color::WHITE);
        // draw origin (the number to be converted)
        let mut text_origin = Text::new(origin, self.font.deref(), 200);
        text_origin.set_fill_color(Color::BLACK);
        text_origin.set_position(Vector2f::new((window.size().x / 2) as f32 - text_origin.global_bounds().width / 2 as f32, 25.0));

        let text_input = Vec::<Text>::with_capacity(8);
        for i in 0..(text_input.capacity()) { // TODO: Wie funktionieren Ranges???
            let wrapped = &player_input.get(i);
            if wrapped.is_some() {
                let text = &player_input.get(i).unwrap().to_string();
                let mut text_input_build = Text::new(text, self.font.deref(), 25);
                text_input_build.set_fill_color(Color::BLACK);
                text_input_build.set_position(Vector2f::new((window.size().x / 4) as f32 + (50 * i) as f32, 500.0));
                window.draw(&text_input_build);
            }
        }

        window.draw(&text_origin);

        // draw the snowman, we all love
        match snowman_state {
            SnowmanStates::Idle => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y)),
            SnowmanStates::Melting(animation_start) => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y * ((current_frame - animation_start) as f32 / self.animation_duration as f32))),
            SnowmanStates::Melted => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, 0.0)),
            _ => println!("No rendering is defined for snowman_state"),
        }

        window.display();
    }
}

fn get_snowman(start_pos_x: f32, start_pos_y: f32, scale_x: f32, scale_y: f32) -> VertexArray {
    let snowman = [
        Vector2f::new(5.0, 8.0),
        Vector2f::new(5.0, 8.0),
        Vector2f::new(3.0, 8.0),
        Vector2f::new(1.0, 10.0),
        Vector2f::new(1.0, 12.0),
        Vector2f::new(3.0, 14.0),
        Vector2f::new(5.0, 14.0),
        Vector2f::new(7.0, 12.0),
        Vector2f::new(7.0, 10.0),
        Vector2f::new(5.0, 8.0),
        Vector2f::new(8.0, 5.0),
        Vector2f::new(8.0, 2.0),
        Vector2f::new(6.0, 0.0),
        Vector2f::new(2.0, 0.0),
        Vector2f::new(0.0, 2.0),
        Vector2f::new(0.0, 5.0),
        Vector2f::new(3.0, 8.0),
    ];

    let mut snowman_builder = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, snowman.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + snowman[0].x * scale_x, start_pos_y - snowman[0].y * scale_y), Color::WHITE, Vector2f::new(0.0, 0.0)));
    for point in snowman {
        snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + point.x * scale_x, start_pos_y - point.y * scale_y), Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_builder
}

impl Renderer {}

pub fn new(snowman_pos: Vector2f, snowman_scale: Vector2f, animation_duration: i32) -> Renderer {
    // load font
    let font = Font::from_file("font.ttf").unwrap();

    Renderer {
        snowman_pos,
        snowman_scale,
        animation_duration,
        font,
    }
}
