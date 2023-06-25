use std::ops::{Add, Deref};

use sfml::graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable, Vertex, VertexBuffer, VertexBufferUsage};
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

const HAT: [Vector2<f32>; 7] = [
    Vector2f::new(0.0, 0.0), Vector2f::new(1.0, 0.0), Vector2f::new(1.0, 2.0),
    Vector2f::new(2.0, 2.0), Vector2f::new(2.0, 0.0), Vector2f::new(3.0, 0.0),
    Vector2f::new(0.0, 0.0)];

pub struct Renderer {
    snowman_pos: Vector2f,
    snowman_scale: Vector2f,
    snowman_idle_amplifier: f32,
    animation_duration: i32,
    hat_left: Vector2f,
    hat_right: Vector2f,
    font: SfBox<Font>,
}

impl Renderer {
    pub(crate) fn render(&mut self, window: &mut RenderWindow, player_input: &Vec<u8>, origin: &String, current_frame: i32, snowman_state: SnowmanStates) {
        window.clear(Color::WHITE);
        // draw origin (the number to be converted)
        let mut text_origin = Text::new(origin, self.font.deref(), 200);
        text_origin.set_fill_color(Color::BLACK);
        text_origin.set_position(Vector2f::new((window.size().x / 2) as f32 - text_origin.global_bounds().width / 2 as f32, 25.0));

        let text_input = Vec::<Text>::with_capacity(8);
        for i in 0..(text_input.capacity()) { // ranges seem to behave strangely
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

        let mut snowman_scale = Vector2f::new(self.snowman_scale.x, self.snowman_scale.y + f32::sin(current_frame as f32 / 150.0) * self.snowman_idle_amplifier);

        // draw the snowman, we all love
        let snowman =
            match snowman_state {
                SnowmanStates::Idle => get_snowman(),
                SnowmanStates::Melting(animation_start) => {
                    // hard overwriting snowman_scale
                    snowman_scale = Vector2f::new(snowman_scale.x, snowman_scale.y - snowman_scale.y * (current_frame - animation_start) as f32 / self.animation_duration as f32);
                    get_snowman()
                },
                SnowmanStates::Melted => {
                    // hard overwriting snowman_scale
                    snowman_scale = Vector2f::new(snowman_scale.x, 0.0);
                    get_snowman()
                },
                SnowmanStates::MorphingIntoAFirTree(animation_start) => {
                    snowman_scale = Vector2f::new(snowman_scale.x, snowman_scale.y + (((current_frame as f32 / 64.0).sin() * 7.0) * ((current_frame as f32 / 64.0).tan() + 1.0 * 3.0) + (((current_frame as f32).sin()) * 0.05)) / self.animation_duration as f32 * (current_frame - animation_start) as f32);
                    morph_into_christmas_tree(current_frame - animation_start, self.animation_duration)
                },
                SnowmanStates::MorphingFromAFirTree(animation_start) => {
                    snowman_scale = Vector2f::new(snowman_scale.x, snowman_scale.y + (((current_frame as f32 / 64.0).sin() * 7.0) * ((current_frame as f32 / 64.0).tan() + 1.0 * 3.0) + (((current_frame as f32).sin()) * 0.05)) / self.animation_duration as f32 * (animation_start - current_frame - animation_start) as f32);
                    morph_from_christmas_tree(current_frame - animation_start, self.animation_duration)
                },
                SnowmanStates::IsFirTree() => {
                    snowman_scale = Vector2f::new(snowman_scale.x, snowman_scale.y + ((current_frame as f32 / 64.0).sin() * 7.0) * ((current_frame as f32 / 64.0).tan() + 1.0 * 3.0) + (((current_frame as f32).sin()) * 0.05));
                    get_christmas_tree()
                },
                _ => {
                    println!("No rendering is defined for snowman_state");
                    EMPTY.to_vec()
                },
            };
        {
            // create VertexBuffer from vertexes
            // +1 because we have to add another white vertex to hide the origin
            let mut offset = 0;
            let mut drawing = VertexBuffer::new(sfml::graphics::PrimitiveType::LINE_STRIP, (snowman.len() + 1) as u32, VertexBufferUsage::STREAM);
            drawing.update(&[Vertex::new(
                Vector2f::new(self.snowman_pos.x + snowman[0].x * snowman_scale.x, self.snowman_pos.y - snowman[0].y * snowman_scale.y),
                Color::WHITE, Vector2f::new(0.0, 0.0))], offset);
            for point in &snowman {
                offset += 1;
                drawing.update(&[Vertex::new(
                    Vector2f::new(self.snowman_pos.x + point.x * snowman_scale.x, self.snowman_pos.y - point.y * snowman_scale.y),
                    Color::BLACK, Vector2f::new(0.0, 0.0))], offset);
            }
            window.draw(&drawing);
        }


        let hat_holding_modifier = Vector2f::new(0.0, 0.0);


        {
            // Hat Magic (simulates gravity)
            // to not have the hat flying, it checks the height of the snowman on two sides
            // using this information, the hat shouldn't have any problems, when the snowman is morphing

            // left part of hat
            let max_left_hat_pos_y = get_max_height_at(self.hat_left.x, &snowman);
            if max_left_hat_pos_y >= self.hat_left.y - 0.055
            { self.hat_left = Vector2f::new(self.hat_left.x, max_left_hat_pos_y) } else { self.hat_left = Vector2f::new(self.hat_left.x, self.hat_left.y - 0.055); }

            // right part of hat
            let max_right_hat_pos_y = get_max_height_at(self.hat_right.x, &snowman);
            if max_right_hat_pos_y >= self.hat_right.y - 0.055
            { self.hat_right = Vector2f::new(self.hat_right.x, max_right_hat_pos_y) } else { self.hat_right = Vector2f::new(self.hat_right.x, self.hat_right.y - 0.055); }

            // draw hat to window
            let mut offset = 0;
            let mut hat = VertexBuffer::new(sfml::graphics::PrimitiveType::LINE_STRIP, (HAT.len() + 1) as u32, VertexBufferUsage::STREAM);
            let modifier = (self.hat_right.y - self.hat_left.y) / (self.hat_right.x - self.hat_left.x);

            hat.update(&[Vertex::new(
                Vector2f::new(
                    self.snowman_pos.x + (hat_holding_modifier.x + self.hat_left.x + HAT[0].x) * snowman_scale.x,
                    self.snowman_pos.y - ((self.hat_left.y + hat_holding_modifier.y + HAT[0].y + (modifier * (HAT[0].x))) * snowman_scale.y),
                ),
                Color::WHITE, Vector2f::new(0.0, 0.0))], offset);
            for point in HAT {
                offset += 1;
                hat.update(&[Vertex::new(
                    Vector2f::new(
                        self.snowman_pos.x + (hat_holding_modifier.x + self.hat_left.x + point.x) * snowman_scale.x,
                        self.snowman_pos.y - ((self.hat_left.y + hat_holding_modifier.y + point.y + (modifier * (point.x))) * snowman_scale.y),
                    ),
                    Color::BLACK, Vector2f::new(0.0, 0.0))], offset);
            }
            window.draw(&hat);
        }

        window.display();
    }
}

fn get_snowman_arm(snowman: &Vec<Vector2f>) -> Vector2f {
    // get x length
    let mut max_x = 0.0;
    for point in snowman {
        if point.x > max_x {
            max_x = point.x;
        }
    }
    // search for arm possibilities
    let mut arm_possibilities = Vec::new();
    for point in snowman {
        if point.x == max_x {
            arm_possibilities.push(point);
        }
    }
    // return arm pos
    if arm_possibilities.len() > 1 {
        arm_possibilities.get(arm_possibilities.len() - 2).unwrap().clone().to_owned()
    } else {
        arm_possibilities.get(0).unwrap().clone().to_owned()
    }
}

fn get_max_height_at(x_pos: f32, snowman: &Vec<Vector2f>) -> f32 {
    let mut max_height = 0.0;
    for i in 0..snowman.len() - 1 {
        let mut j = i + 1;
        if i == snowman.len() { // to compare the last with the first
            j = 0;
        }
        // check whether one or both of the points have a higher y value than our max
        if snowman[i].y > max_height || snowman[j].y > max_height {
            /*
            check whether the x position, we are searching for
            is or is between snowman[i] and / or snowman[j]
             */
            if snowman[i].x == x_pos { max_height = snowman[i].y } else if snowman[j].x == x_pos { max_height = snowman[j].y } else if ((snowman[i].x > x_pos) && (x_pos > snowman[j].x)) || ((snowman[i].x < x_pos) && (x_pos < snowman[j].x)) {
                // like doing an linear equation (y=mx+t)
                let modifier = (snowman[j].y - snowman[i].y) / (snowman[j].x - snowman[i].x);
                // check whether the intersection is really higher than max_height
                if snowman[i].y + modifier * (x_pos - snowman[i].x) > max_height {
                    max_height = snowman[i].y + modifier * (x_pos - snowman[i].x);
                }
            }
        }
    }
    return max_height;
}

fn morph_into_christmas_tree(animation_frame: i32, animation_duration: i32) -> Vec<Vector2f> {
    let mut christmas_tree = Vec::with_capacity(17);
    for point in add_vec_array(SNOWMAN, mul_vec_array(
        div_vec_array_of_number(sub_vec_array(CHRISTMAS_TREE, SNOWMAN), animation_duration as f32),
        animation_frame as f32)) {
        christmas_tree.push(point);
    }
    return christmas_tree
}

fn morph_from_christmas_tree(animation_frame: i32, animation_duration: i32) -> Vec<Vector2f> {
    let mut snowman = Vec::with_capacity(17);
    for point in add_vec_array(CHRISTMAS_TREE, mul_vec_array(
        div_vec_array_of_number(sub_vec_array(SNOWMAN, CHRISTMAS_TREE), animation_duration as f32),
        animation_frame as f32)) {
        snowman.push(point);
    }
    return snowman
}

fn get_christmas_tree() -> Vec<Vector2f> {
    let mut christmas_tree = Vec::with_capacity(17);
    for point in CHRISTMAS_TREE {
        christmas_tree.push(point);
    }
    return christmas_tree
}

fn get_snowman() -> Vec<Vector2f> {
    let mut snowman = Vec::with_capacity(17);
    for point in SNOWMAN {
        snowman.push(point)
    }
    return snowman
}

fn add_vec_array(vecarray1: [Vector2f; 17], vecarray2: [Vector2f; 17]) -> [Vector2f; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] + vecarray2[i];
        i += 1;
    }
    return result
}

fn sub_vec_array(vecarray1: [Vector2f; 17], vecarray2: [Vector2f; 17]) -> [Vector2f; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] - vecarray2[i];
        i += 1;
    }
    return result
}

fn div_vec_array(vecarray1: [Vector2f; 17], vecarray2: [Vector2f; 17]) -> [Vector2f; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i].x = vecarray1[i].x / vecarray2[i].x;
        result[i].y = vecarray1[i].y / vecarray2[i].y;
        i += 1;
    }
    return result
}

fn div_vec_array_of_number(vecarray1: [Vector2f; 17], number: f32) -> [Vector2f; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray1.len() {
        result[i] = vecarray1[i] / number;
        i += 1;
    }
    return result
}

fn mul_vec_array(vecarray: [Vector2f; 17], multiplier: f32) -> [Vector2f; 17] {
    let mut result = EMPTY;
    let mut i: usize = 0;
    while i < vecarray.len() {
        result[i] = vecarray[i] * multiplier;
        i += 1;
    }
    return result
}

impl Renderer {}

pub fn new(snowman_pos: Vector2f, animation_duration: i32) -> Renderer {
    // load font
    let font = Font::from_file("font.ttf").unwrap();

    Renderer {
        snowman_pos,
        snowman_scale: Vector2f::new(25.0, 25.0),
        snowman_idle_amplifier: 1.5,
        animation_duration,
        hat_left: Vector2f::new(3.5, 20.0),
        hat_right: Vector2f::new(6.5, 20.0),
        font,
    }
}
