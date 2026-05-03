use opencv::core::Rect;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub score: f32,
    pub rect: Rect,
}

#[derive(Debug, Clone)]
pub struct UiTemplate {
    pub name: String,
    pub image_path: String,
    pub threshold: f32,
    pub search_region: Option<Rect>,
}
