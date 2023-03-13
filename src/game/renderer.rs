use std::ops::Deref;

use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable, Vertex, VertexArray};
use sfml::SfBox;
use sfml::system::{Vector2, Vector2f};

use crate::game::snowman_states::SnowmanStates;

const SNOWMAN: [Vector2<f32>; 17] = [
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

const CHRISTMAS_TREE: [Vector2<f32>; 17] = [
    Vector2f::new(4.0, 2.0 * 4.0 + 2.0),
    Vector2f::new(7.0, 1.0 * 4.0 + 2.0),
    Vector2f::new(4.0, 1.0 * 4.0 + 2.0),
    Vector2f::new(7.0, 2.0),
    Vector2f::new(4.0, 2.0), // right
    Vector2f::new(4.0, 0.0), // tree trunk
    Vector2f::new(3.0, 0.0), // tree trunk
    Vector2f::new(3.0, 2.0), // left
    Vector2f::new(0.0, 2.0),
    Vector2f::new(3.0, 1.0 * 4.0 + 2.0),
    Vector2f::new(0.0, 1.0 * 4.0 + 2.0),
    Vector2f::new(3.0, 2.0 * 4.0 + 2.0),
    Vector2f::new(0.0, 2.0 * 4.0 + 2.0),
    Vector2f::new(3.5, 14.0),
    Vector2f::new(7.0, 2.0 * 4.0 + 2.0),
    Vector2f::new(4.0, 2.0 * 4.0 + 2.0),
    Vector2f::new(7.0, 1.0 * 4.0 + 2.0),
];

const EMPTY: [Vector2<f32>; 17] = [Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
    Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
    Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
    Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
    Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
    Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0), Vector2f::new(0.0, 0.0),
];

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
            let mut text = "_".to_string();
            if wrapped.is_some() {
                text = player_input.get(i).unwrap().to_string();
            }
            let mut text_input_build = Text::new(text.deref(), self.font.deref(), 25);
            text_input_build.set_fill_color(Color::BLACK);
            text_input_build.set_position(Vector2f::new(window.size().x as f32 / 3.25 + (70 * i) as f32, 500.0));
            window.draw(&text_input_build);

            let mut help_text = Text::new(format!("{}", (2_i32.pow((7 - i) as u32))).deref(), self.font.deref(), 25);
            help_text.set_fill_color(Color::BLACK);
            help_text.set_position(Vector2f::new(window.size().x as f32 / 3.25 + (70.0 * i as f32), 550.0));
            window.draw(&help_text);
        }

        window.draw(&text_origin);

        // draw the snowman, we all love
        match snowman_state {
            SnowmanStates::Idle => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y)),
            SnowmanStates::Melting(animation_start) => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y - self.snowman_scale.y * ((current_frame - animation_start) as f32 / self.animation_duration as f32))),
            SnowmanStates::Melted => window.draw(&get_snowman(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, 0.0)),
            SnowmanStates::MorphingIntoAFirTree(animation_start) => window.draw(&morph_into_christmas_tree(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y, current_frame - animation_start, self.animation_duration)),
            SnowmanStates::MorphingFromAFirTree(animation_start) => window.draw(&morph_from_christmas_tree(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y, current_frame - animation_start, self.animation_duration)),
            SnowmanStates::IsFirTree() => window.draw(&get_christmas_tree(self.snowman_pos.x, self.snowman_pos.y, self.snowman_scale.x, self.snowman_scale.y)),
            _ => println!("No rendering is defined for snowman_state"),
        }

        window.display();
    }
}

fn morph_into_christmas_tree(start_pos_x: f32, start_pos_y: f32, length_unit_x: f32, length_unit_y: f32, animation_frame: i32, animation_duration: i32) -> VertexArray {
    let mut christmas_tree_generator = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, SNOWMAN.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    let mut first = true;
    for point in add_vec_array(SNOWMAN, mul_vec_array(
        div_vec_array_of_number(sub_vec_array(CHRISTMAS_TREE, SNOWMAN), animation_duration as f32),
        animation_frame as f32)) {
        if first {
            christmas_tree_generator.append(&Vertex::new(
                Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y),
                Color::WHITE, Vector2f::new(0.0, 0.0)));
            first = false;
        }
        christmas_tree_generator.append(&Vertex::new(
            Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y),
            Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return christmas_tree_generator
}

fn morph_from_christmas_tree(start_pos_x: f32, start_pos_y: f32, length_unit_x: f32, length_unit_y: f32, animation_frame: i32, animation_duration: i32) -> VertexArray {
    let mut snowman_generator = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, SNOWMAN.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    let mut first = true;
    for point in add_vec_array(CHRISTMAS_TREE, mul_vec_array(
        div_vec_array_of_number(sub_vec_array(SNOWMAN, CHRISTMAS_TREE), animation_duration as f32),
        animation_frame as f32)) {
        if first {
            snowman_generator.append(&Vertex::new(
                Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y),
                Color::WHITE, Vector2f::new(0.0, 0.0)));
            first = false;
        }
        snowman_generator.append(&Vertex::new(
            Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y),
            Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_generator
}

fn get_christmas_tree(start_pos_x: f32, start_pos_y: f32, length_unit_x: f32, length_unit_y: f32) -> VertexArray {
    let mut snowman_builder = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, SNOWMAN.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + CHRISTMAS_TREE[0].x * length_unit_x, start_pos_y - CHRISTMAS_TREE[0].y * length_unit_y), Color::WHITE, Vector2f::new(0.0, 0.0)));
    for point in CHRISTMAS_TREE {
        snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y), Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_builder
}

fn get_snowman(start_pos_x: f32, start_pos_y: f32, length_unit_x: f32, length_unit_y: f32) -> VertexArray {
    let mut snowman_builder = VertexArray::new(sfml::graphics::PrimitiveType::LINE_STRIP, SNOWMAN.len());

    // ein Punkt ist zwar doppelt vorhanden,
    // dafür wird keine schwarze diagonale Linie von (0/0) nach snowman[0] gezeichnet
    snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + SNOWMAN[0].x * length_unit_x, start_pos_y - SNOWMAN[0].y * length_unit_y), Color::WHITE, Vector2f::new(0.0, 0.0)));
    for point in SNOWMAN {
        snowman_builder.append(&Vertex::new(Vector2f::new(start_pos_x + point.x * length_unit_x, start_pos_y - point.y * length_unit_y), Color::BLACK, Vector2f::new(0.0, 0.0)))
    }
    return snowman_builder
}

fn add_vec_array(vecarray1: [Vector2<f32>; 17], vecarray2: [Vector2<f32>; 17]) -> [Vector2<f32>; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] + vecarray2[i];
        i += 1;
    }
    return result
}

fn sub_vec_array(vecarray1: [Vector2<f32>; 17], vecarray2: [Vector2<f32>; 17]) -> [Vector2<f32>; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] - vecarray2[i];
        i += 1;
    }
    return result
}

fn div_vec_array(vecarray1: [Vector2<f32>; 17], vecarray2: [Vector2<f32>; 17]) -> [Vector2<f32>; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] / vecarray2[i];
        i += 1;
    }
    return result
}

fn div_vec_array_of_number(vecarray1: [Vector2<f32>; 17], number: f32) -> [Vector2<f32>; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] / number;
        i += 1;
    }
    return result
}

fn mul_vec_array(vecarray: [Vector2<f32>; 17], multiplier: f32) -> [Vector2<f32>; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray.len() {
        result[i] = vecarray[i] * multiplier;
        i += 1;
    }
    return result
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
