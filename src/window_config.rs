use macroquad::window::Conf;

pub fn window_conf() -> Conf {
    // Get the screen dimensions
    let window_width = 800; // Your desired window width
    let window_height = 600; // Your desired window height

    Conf {
        window_title: "Rhythm Visualizer".to_owned(),
        window_width,
        window_height,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
