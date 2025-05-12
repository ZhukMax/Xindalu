mod interactions;
pub mod stats;

use macroquad::prelude::*;
use crate::components::Pet;

pub struct Button {
    pub rect: Rect,
    pub label: &'static str,
    pub action: fn(&mut Pet),
}

pub fn handle_interactions(pet: &mut Pet) {
    // // Feed, if there is energy
    // if is_key_pressed(KeyCode::F) && pet.energy >= 5.0 {
    //     pet.hunger += 10.0;
    //     pet.energy -= 5.0; // Feeding wastes energy
    //     pet.hunger = pet.hunger.clamp(0.0, 100.0);
    //     pet.energy = pet.energy.clamp(0.0, 100.0);
    // }
    // 
    // // Play
    // if is_key_pressed(KeyCode::P) && pet.energy >= 15.0 {
    //     pet.happiness += 15.0;
    //     pet.energy -= 15.0;
    //     pet.happiness = pet.happiness.clamp(0.0, 100.0);
    //     pet.energy = pet.energy.clamp(0.0, 100.0);
    // }
    let buttons = vec![
        Button {
            rect: Rect::new(50.0, 500.0, 150.0, 50.0), // Feed: слева
            label: "Feed",
            action: |pet| {
                if pet.energy >= 5.0 {
                    pet.hunger += 10.0;
                    pet.energy -= 5.0;
                    pet.hunger = pet.hunger.clamp(0.0, 100.0);
                    pet.energy = pet.energy.clamp(0.0, 100.0);
                }
            },
        },
        Button {
            rect: Rect::new(600.0, 500.0, 150.0, 50.0), // Play: справа
            label: "Play",
            action: |pet| {
                if pet.energy >= 15.0 {
                    pet.happiness += 15.0;
                    pet.energy -= 15.0;
                    pet.happiness = pet.happiness.clamp(0.0, 100.0);
                    pet.energy = pet.energy.clamp(0.0, 100.0);
                }
            },
        },
    ];

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        for button in buttons {
            if button.rect.contains(Vec2::new(mouse_x, mouse_y)) {
                (button.action)(pet);
            }
        }
    }
}

pub fn render_pet(pet: &Pet, texture: &Texture2D) {
    // Draw a cat (scale for visibility)
    draw_texture_ex(
        texture,
        350.0,
        250.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(64.0, 64.0)), // Scaling 32x32 to 64x64
            ..Default::default()
        },
    );

    // Displaying stats
    draw_text(&format!("Hunger: {:.1}", pet.hunger), 20.0, 20.0, 30.0, WHITE);
    draw_text(&format!("Happiness: {:.1}", pet.happiness), 20.0, 50.0, 30.0, WHITE);
    draw_text(&format!("Health: {:.1}", pet.health), 20.0, 80.0, 30.0, WHITE);
    draw_text(&format!("Energy: {:.1}", pet.energy), 20.0, 110.0, 30.0, WHITE);

    let buttons = vec![
        Button {
            rect: Rect::new(50.0, 500.0, 150.0, 50.0),
            label: "Feed",
            action: |_| {},
        },
        Button {
            rect: Rect::new(600.0, 500.0, 150.0, 50.0),
            label: "Play",
            action: |_| {},
        },
    ];

    for button in buttons {
        // Draw a rectangle for the button
        draw_rectangle(
            button.rect.x,
            button.rect.y,
            button.rect.w,
            button.rect.h,
            GRAY,
        );
        // Draw the button text
        draw_text(
            button.label,
            button.rect.x + 20.0,
            button.rect.y + 35.0,
            30.0,
            WHITE,
        );
    }
}
