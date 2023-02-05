use sfml::graphics::{Color, RenderTarget, RenderWindow, Vertex, VertexArray};
use sfml::system::Vector2f;

use crate::game::snowman_states::SnowmanStates;

pub struct Renderer {
    snowman_pos: Vector2f,
    snowman_scale: Vector2f,
    animation_duration: i32,
}

impl Renderer {
    pub(crate) fn render(&self, window: &mut RenderWindow, player_input: &Vec<char>, current_frame: i32, snowman_state: SnowmanStates) {
        window.clear(Color::WHITE);
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
        Vector2f::new(5.0, 6.0),
        Vector2f::new(5.0, 6.0),
        Vector2f::new(3.0, 6.0),
        Vector2f::new(1.0, 4.0),
        Vector2f::new(1.0, 2.0),
        Vector2f::new(3.0, 0.0),
        Vector2f::new(5.0, 0.0),
        Vector2f::new(7.0, 2.0),
        Vector2f::new(7.0, 4.0),
        Vector2f::new(5.0, 6.0),
        Vector2f::new(8.0, 9.0),
        Vector2f::new(8.0, 12.0),
        Vector2f::new(6.0, 14.0),
        Vector2f::new(2.0, 14.0),
        Vector2f::new(0.0, 12.0),
        Vector2f::new(0.0, 9.0),
        Vector2f::new(3.0, 6.0),
    ];

    let mut snowman_builder = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, snowman.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + snowman[0].x * scale_x, start_pos_y + snowman[0].y * scale_y), Color::WHITE, Vector2f::new(0.0, 0.0)));
    for point in snowman {
        snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + point.x * scale_x, start_pos_y + point.y * scale_y), Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_builder
}

impl Renderer {}

pub fn new(snowman_pos: Vector2f, snowman_scale: Vector2f, animation_duration: i32) -> Renderer {
    Renderer {
        snowman_pos,
        snowman_scale,
        animation_duration,
    }
}
