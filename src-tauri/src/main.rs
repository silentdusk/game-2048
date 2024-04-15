// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Mutex,
    thread,
};
use tauri::{Manager, State};

mod game;

use game::Game;

struct GameState {
    game: Mutex<Game>,
    app_dir: Mutex<Option<PathBuf>>,
}

#[tauri::command]
fn new_game(game_state: State<GameState>) -> Game {
    game_state.game.lock().unwrap().reset();
    save_game_state(&game_state);
    *game_state.game.lock().unwrap()
}

#[tauri::command]
fn get_game_state(game_state: State<GameState>) -> Game {
    *game_state.game.lock().unwrap()
}

#[tauri::command]
fn game_event_listener(game_event: u8, game_state: State<GameState>) -> Game {
    game_state.game.lock().unwrap().event(game_event);
    save_game_state(&game_state);
    *game_state.game.lock().unwrap()
}

fn save_game_state(game_state: &GameState) {
    if let Some(ref app_dir) = *game_state.app_dir.lock().unwrap() {
        let json_path: PathBuf = app_dir.join("game_data.json");
        let game_struct = *game_state.game.lock().unwrap();
        thread::spawn(move || {
            if game_struct.lock {
                let _ = fs::remove_file(json_path);
            } else {
                let game_data = serde_json::to_string(&game_struct)
                    .expect("Failed converting struct to string");
                let mut json_file = File::create(json_path).expect("File creation failed");
                json_file
                    .write_all(game_data.as_bytes())
                    .expect("Writing failed");
            }
        });
    }
}

fn main() {
    tauri::Builder::default()
        .manage(GameState {
            game: Mutex::new(Game::new()),
            app_dir: Default::default(),
        })
        .setup(|app| {
            let app_handle = app.handle();
            let game_state: State<GameState> = app_handle.state();
            let app_dir = app_handle
                .path_resolver()
                .app_data_dir()
                .expect("The app directory should exist");
            *game_state.app_dir.lock().unwrap() = Some(app_dir.clone());

            let json_path = app_dir.join("game_data.json");
            if json_path.exists() {
                if let Ok(game_data) = std::fs::read_to_string(json_path) {
                    let game: Game = serde_json::from_str(&game_data)?;
                    *game_state.game.lock().unwrap() = game;
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            new_game,
            game_event_listener,
            get_game_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
