use sfml::graphics::{Color, RenderTarget, RenderWindow, Vertex, VertexArray};
use sfml::system::Vector2f;

use crate::game::snowman_states::SnowmanStates;

pub struct Renderer {}

impl Renderer {
    pub(crate) fn render(&self, window: &mut RenderWindow, player_input: &Vec<char>, current_frame: i32, snowman_state: SnowmanStates) {
        window.clear(Color::WHITE);
        // draw the snowman, we all love
        match snowman_state {
            SnowmanStates::Idle => window.draw(&get_snowman(0.0, 0.0, 25.0, 25.0)),
            _ => window.draw(&get_snowman(0.0, 0.0, 25.0, 25.0)),
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
    for point in snowman {
        snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + point.x * scale_x, start_pos_y + point.y * scale_y), Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_builder
}

fn vertex_from_point(x: f32, y: f32) -> Vertex {
    return Vertex::new(Vector2f::new(x, y), Color::BLACK, Vector2f::new(0.0, 0.0))
}

impl Renderer {}

pub fn new() -> Renderer { Renderer {} }
