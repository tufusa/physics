impl super::Pyramid {
    pub(crate) const STEP: u32 = 100;
    pub(crate) const BASE_Y: f32 = -super::Window::SIZE.y / 2. + super::Wall::VISIBLE_WIDTH;
}
