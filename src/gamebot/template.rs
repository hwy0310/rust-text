// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use opencv::core::Rect;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct MatchResult {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub score: f32,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub rect: Rect,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct UiTemplate {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub name: String,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub image_path: String,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub threshold: f32,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub search_region: Option<Rect>,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
