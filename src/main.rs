use macroquad::prelude::*;
mod components;
mod systems;
mod resources;

use components::Pet;
use resources::GameState;
use systems::{stats::update_pet_stats, handle_interactions, render_pet};

#[macroquad::main("Xindalu")]
async fn main() {
    let mut pet = Pet {
        hunger: 50.0,
        happiness: 50.0,
        health: 80.0,
        energy: 70.0,
    };
    let game_state = GameState { is_running: true };
    
    // Загружаем текстуру котика
    let pet_texture = load_texture("assets/pet.png").await.unwrap();

    loop {
        if !game_state.is_running {
            break;
        }

        // Обновление
        update_pet_stats(&mut pet, get_frame_time());
        handle_interactions(&mut pet);

        // Рендеринг
        clear_background(BLACK);
        render_pet(&pet, &pet_texture);

        next_frame().await;
    }
}
