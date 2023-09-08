#[derive(Debug, Copy, Clone)]
pub struct Pin {
    pub state: bool
}
impl Pin {
    pub fn new() -> Self {
        Self {
            state: false
        }
    }
}