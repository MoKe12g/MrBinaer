use sfml::graphics::RenderWindow;
use sfml::window::Style;

mod game;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "MrBinaer",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_mouse_cursor_visible(true);
    window.set_vertical_sync_enabled(true); // VSync

    loop {
        // rand ist so schlau, dass es den Ziel-Typen erkennt
        let mut game = game::new(rand::random(), 120);
        game.game_loop(&mut window);
    }
}
