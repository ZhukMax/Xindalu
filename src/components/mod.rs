#[derive(Debug)]
pub struct Pet {
    pub hunger: f32,    // 0..100
    pub happiness: f32, // 0..100
    pub health: f32,    // 0..100, depends on hunger
    pub energy: f32,    // 0..100, depends on happiness and actions
}
