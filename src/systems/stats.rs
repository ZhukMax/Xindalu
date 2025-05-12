use crate::components::Pet;

pub fn update_pet_stats(pet: &mut Pet, delta: f32) {
    // Renewal of hunger and happiness
    pet.hunger -= 1.25 * delta;
    pet.happiness -= 0.75 * delta;

    // Health Renewal: Drops if hunger < 20
    if pet.hunger < 20.0 {
        pet.health -= 2.5 * delta;
    } else {
        pet.health += 0.5 * delta; // Slow recovery when full
    }

    // Energy Renewal: Decreases over time, restores when happiness > 70
    pet.energy -= 1.0 * delta;
    if pet.happiness > 70.0 {
        pet.energy += 0.75 * delta;
    }

    // Limiting values
    pet.hunger = pet.hunger.clamp(0.0, 100.0);
    pet.happiness = pet.happiness.clamp(0.0, 100.0);
    pet.health = pet.health.clamp(0.0, 100.0);
    pet.energy = pet.energy.clamp(0.0, 100.0);
}
