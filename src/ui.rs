use macroquad::{
    color::{ BLACK, DARKGRAY, LIGHTGRAY, WHITE },
    input::{ is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton },
    prelude::Color,
    shapes::{ draw_line, draw_rectangle },
    text::draw_text,
    texture::{ draw_texture, load_texture },
    window::{ clear_background, screen_height, screen_width },
};
use crate::structs::{ Assets, SongSelectionState, FloatingText };
use crate::constants::*;
use std::fs;

/// Load all UI assets, such as textures and fonts.
///
/// UI assets are loaded asynchronously, so this function returns a future.
/// The future resolves to an `Assets` struct containing all the loaded assets.
///
/// The assets are loaded from the `src/assets/images/` directory.
///
/// * `menu_background` is the background image for the main menu.
/// * `start_button` is the start button image.
pub async fn load_ui_assets() -> Assets {
    let menu_background = load_texture("src/assets/images/main_menu.png").await.unwrap();
    let start_button = load_texture("src/assets/images/start_button.png").await.unwrap();

    Assets {
        menu_background,
        start_button,
    }
}

/// Draw the main menu.
///
/// The main menu is drawn in the center of the screen and consists of a background image,
/// a start button, and a quit button.
///
/// The `assets` parameter is an `Assets` struct containing the loaded UI assets.
///
/// The `start_game` parameter is a boolean indicating whether the player has pressed the start button.
///
/// If the player has pressed the start button, the function returns `true`.
///
/// If the player has not pressed the start button, the function returns `false`.
///
/// The function also draws the UI elements, such as the background image and the start button.
pub fn draw_menu(assets: &Assets) -> bool {
    let mut start_game = false;

    clear_background(BLACK);

    draw_texture(&assets.menu_background, 0.0, 0.0, WHITE);

    let start_button = &assets.start_button;
    let button_x = screen_width() / 2.0 - start_button.width() / 2.0;
    let button_y = screen_height() / 2.0;

    draw_texture(start_button, button_x, button_y, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if
            mouse_pos.0 >= button_x &&
            mouse_pos.0 <= button_x + start_button.width() &&
            mouse_pos.1 >= button_y &&
            mouse_pos.1 <= button_y + start_button.height()
        {
            start_game = true;
        }
    }

    start_game
}

/// Implement the `SongSelectionState` struct.
///
/// The `SongSelectionState` struct is used to store the state of the song selection menu.
///
/// The `new` function creates a new `SongSelectionState` instance with default values.
impl SongSelectionState {
    pub fn new() -> Self {
        Self {
            scroll_pos: 0.0,
            selected_song: None,
        }
    }
}

/// Draw the song selection menu.
///
/// The song selection menu is drawn in the center of the screen and consists of a list of songs.
///
/// The `state` parameter is a `SongSelectionState` struct containing the state of the song selection menu.
///
/// The `songs` parameter is a vector of strings containing the names of all the songs.
///
/// The `selected_song` parameter is an `Option<String>` containing the selected song, if any.
///
/// If the player has selected a song, the function returns `Some(song)`, where `song` is the selected song.
///
/// If the player has not selected a song, the function returns `None`.
///
/// The function also draws the UI elements, such as the list of songs and the selected song.
pub fn draw_choose_audio(state: &mut SongSelectionState, songs: &[String]) -> Option<String> {
    clear_background(BLACK);

    draw_text("List of audio files", 20.0, 50.0, 30.0, DARKGRAY);

    let screen_h = screen_height();

    if is_key_down(KeyCode::Down) {
        state.scroll_pos += 5.0;
    }
    if is_key_down(KeyCode::Up) {
        state.scroll_pos -= 5.0;
    }
    if state.scroll_pos < 0.0 {
        state.scroll_pos = 0.0;
    }

    for (i, song) in songs.iter().enumerate() {
        let button_x = 20.0;
        let button_y = 80.0 + (i as f32) * SONG_ENTRY_HEIGHT - state.scroll_pos;

        if button_y > SONG_ENTRY_HEIGHT && button_y < screen_h - SONG_ENTRY_HEIGHT {
            let button_width = 300.0;
            let button_height = 35.0;
            draw_rectangle(
                button_x - 5.0,
                button_y - 25.0,
                button_width,
                button_height,
                Color::new(1.0, 1.0, 1.0, 0.3)
            );
            let song_name = song.split('/').last().unwrap();
            draw_text(song_name, button_x, button_y, FONT_SIZE, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if
                    mouse_pos.0 >= button_x - 5.0 &&
                    mouse_pos.0 <= button_x - 5.0 + button_width &&
                    mouse_pos.1 >= button_y - 25.0 &&
                    mouse_pos.1 <= button_y - 25.0 + button_height
                {
                    return Some(song.clone());
                }
            }
        }
    }

    None
}

/// Load all songs from the assets directory.
///
/// The function reads the `src/assets/music/` directory and loads all the songs with the `.mp3` extension.
///
/// The function returns a vector of strings containing the names of all the songs.
pub fn load_songs_from_assets() -> Vec<String> {
    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir("src/assets/music/") {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "mp3" {
                    let full_path = entry.path().to_string_lossy().to_string();
                    songs.push(full_path.clone());
                    println!("Loaded song: {}", full_path.clone());
                }
            }
        }
    }
    songs
}

/// Draw a loading bar.
///
/// The loading bar is drawn in the center of the screen.
///
/// The `elapsed_time` parameter is the elapsed time since the loading started.
///
/// The function draws a black rectangle with a white border and a loading bar inside.
///
/// The loading bar is a gray rectangle that fills up the black rectangle as the elapsed time increases.
pub fn draw_loading_bar(elapsed_time: f32) {
    let scr_width = screen_width();
    let scr_height = screen_height();

    draw_rectangle(0.0, 0.0, scr_width, scr_height, BLACK);

    draw_text("Loading...", scr_width / 2.0 - 70.0, scr_height / 2.0 - 50.0, 30.0, WHITE);

    let bar_width = 300.0;
    let bar_height = 30.0;
    let bar_x = scr_width / 2.0 - bar_width / 2.0;
    let bar_y = scr_height / 2.0;

    let progress = (elapsed_time % 2.0) / 2.0;

    draw_rectangle(bar_x, bar_y, bar_width, bar_height, WHITE);
    draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, LIGHTGRAY);
}

/// Draw the score.
///
/// The score is drawn in the top right corner of the screen.
///
/// The `score` parameter is the current score.
///
/// The function draws a black rectangle with a white border and the score inside.
pub fn draw_score(score: i32) {
    let score_text = format!("Score: {}", score);

    draw_text(&score_text, DRAW_SCORE_X + 2.0, DRAW_SCORE_Y + 2.0, SCORE_FONT_SIZE, DARKGRAY);

    draw_text(&score_text, DRAW_SCORE_X, DRAW_SCORE_Y, SCORE_FONT_SIZE, GOLD_COLOR);
}

/// Draw the floating texts.
///
/// The `floating_texts` parameter is a vector of `FloatingText` structs containing the texts to draw.
///
/// The `elapsed` parameter is the elapsed time since the game started.
///
/// The function draws each text in the vector with a y offset based on the elapsed time.
pub fn draw_floating_texts(floating_texts: &mut Vec<FloatingText>, elapsed: f64) {
    floating_texts.retain(|text| {
        let time_since_spawn = elapsed - text.spawn_time;
        if time_since_spawn < text.duration {
            let y_offset = (time_since_spawn * 30.0) as f32;
            let alpha = 1.0 - ((time_since_spawn / text.duration) as f32);

            let color = Color::new(1.0, 0.0, 0.0, alpha);

            draw_text(&text.text, text.position.x, text.position.y - y_offset, FONT_SIZE, color);
            true
        } else {
            false
        }
    });
}

/// Draw the background.
///
/// The background is a gradient of blue and purple colors.
///
/// The `elapsed` parameter is the elapsed time since the game started.
///
/// The function draws the gradient based on the elapsed time.
pub fn draw_background(width: f32, height: f32, elapsed: f64) {
    let color1 = Color::new(0.1, 0.1, 0.3, 1.0);
    let color2 = Color::new(0.2, 0.2, 0.5, 1.0);
    let offset = (elapsed.sin() as f32) * 0.1;

    for y in 0..height as i32 {
        let t = (y as f32) / height + offset;
        let blend_color = Color::new(
            color1.r * (1.0 - t) + color2.r * t,
            color1.g * (1.0 - t) + color2.g * t,
            color1.b * (1.0 - t) + color2.b * t,
            1.0
        );
        draw_line(0.0, y as f32, width, y as f32, 1.0, blend_color);
    }
}
