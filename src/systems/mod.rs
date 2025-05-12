use macroquad::prelude::*;
use crate::components::Pet;

pub fn update_pet_stats(pet: &mut Pet, delta: f32) {
    pet.hunger -= 5.0 * delta; // Голод растет
    pet.happiness -= 3.0 * delta; // Счастье падает
    pet.hunger = pet.hunger.clamp(0.0, 100.0);
    pet.happiness = pet.happiness.clamp(0.0, 100.0);
}

pub fn handle_interactions(pet: &mut Pet) {
    if is_key_pressed(KeyCode::F) { // Кормить
        pet.hunger += 10.0;
        pet.hunger = pet.hunger.clamp(0.0, 100.0);
    }
    if is_key_pressed(KeyCode::P) { // Играть
        pet.happiness += 10.0;
        pet.happiness = pet.happiness.clamp(0.0, 100.0);
    }
}

pub fn render_pet(pet: &Pet, texture: &Texture2D) {
    // Рисуем котика (масштабируем для видимости)
    draw_texture_ex(
        texture,
        350.0,
        250.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(64.0, 64.0)), // Масштабируем 32x32 до 64x64
            ..Default::default()
        },
    );

    // Отображение статов
    draw_text(&format!("Hunger: {:.1}", pet.hunger), 20.0, 20.0, 30.0, WHITE);
    draw_text(&format!("Happiness: {:.1}", pet.happiness), 20.0, 50.0, 30.0, WHITE);
}
