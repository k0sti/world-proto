#[derive(Clone, Copy)]
pub enum AnimationType {
    Rotate,
    Scale,
    Morph,
    Complex,
}

impl AnimationType {
    pub fn next(&self) -> Self {
        match self {
            AnimationType::Rotate => AnimationType::Scale,
            AnimationType::Scale => AnimationType::Morph,
            AnimationType::Morph => AnimationType::Complex,
            AnimationType::Complex => AnimationType::Rotate,
        }
    }
}