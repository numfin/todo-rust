use serde::Serialize;

#[derive(Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub size: usize,
}
