use crate::components::Pet;

pub fn feed(pet: &mut Pet) {
    pet.hunger += 10.0;
    pet.energy -= 5.0; // Feeding wastes energy
    pet.hunger = pet.hunger.clamp(0.0, 100.0);
    pet.energy = pet.energy.clamp(0.0, 100.0);
}

pub fn play(pet: &mut Pet) {
    pet.hunger += 10.0;
    pet.energy -= 5.0; // Feeding wastes energy
    pet.hunger = pet.hunger.clamp(0.0, 100.0);
    pet.energy = pet.energy.clamp(0.0, 100.0);
}
