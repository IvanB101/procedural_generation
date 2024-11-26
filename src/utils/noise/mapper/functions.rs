pub enum Functions {
    Easing(EasingFunctions),
}

pub enum EasingFunctions {}

pub fn change_range(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}
