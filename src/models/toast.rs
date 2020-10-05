#[derive(Debug, Clone, Default)]
pub struct Toast {
    pub is_visible: bool,
    pub title: Option<String>,
    pub content: Option<String>,
}