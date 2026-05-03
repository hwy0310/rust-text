// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::template::{MatchResult, UiTemplate};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use opencv::{core, imgcodecs, imgproc, prelude::*};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::collections::HashMap;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct TemplateDetector {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    cache: HashMap<String, Mat>,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl TemplateDetector {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn new() -> Self {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Self {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            cache: HashMap::new(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn ensure_loaded(&mut self, t: &UiTemplate) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if self.cache.contains_key(&t.name) {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Ok(());
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mat = imgcodecs::imread(&t.image_path, imgcodecs::IMREAD_COLOR)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Vision(format!("模板读取失败 {}: {e}", t.image_path)))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if mat.empty() {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Err(BotError::Vision(format!(
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                "模板为空或路径错误: {}",
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                t.image_path
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            )));
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.cache.insert(t.name.clone(), mat);
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(())
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn locate(&mut self, frame: &Mat, t: &UiTemplate) -> Result<Option<MatchResult>, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.ensure_loaded(t)?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let tpl = self
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .cache
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .get(&t.name)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .ok_or_else(|| BotError::Vision(format!("模板未加载: {}", t.name)))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let roi = if let Some(r) = t.search_region {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Mat::roi(frame, r).map_err(|e| BotError::Vision(format!("ROI 裁剪失败: {e}")))?
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        } else {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            frame.clone().into()
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        };

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let result_cols = roi.cols() - tpl.cols() + 1;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let result_rows = roi.rows() - tpl.rows() + 1;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if result_cols <= 0 || result_rows <= 0 {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Ok(None);
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut result = Mat::zeros(result_rows, result_cols, core::CV_32FC1)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Vision(format!("创建结果矩阵失败: {e}")))?
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .to_mat()
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Vision(format!("to_mat 失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        imgproc::match_template(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            &roi,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            tpl,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            &mut result,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            imgproc::TM_CCOEFF_NORMED,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            &Mat::default(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        )
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        .map_err(|e| BotError::Vision(format!("match_template 失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut min_val = 0.0;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut max_val = 0.0;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut min_loc = core::Point::default();
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut max_loc = core::Point::default();

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        core::min_max_loc(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            &result,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Some(&mut min_val),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Some(&mut max_val),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Some(&mut min_loc),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Some(&mut max_loc),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            &Mat::default(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        )
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        .map_err(|e| BotError::Vision(format!("min_max_loc 失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let score = max_val as f32;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if score < t.threshold {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Ok(None);
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let base = t.search_region.unwrap_or(core::Rect::new(0, 0, 0, 0));
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let rect = core::Rect::new(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            max_loc.x + base.x,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            max_loc.y + base.y,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            tpl.cols(),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            tpl.rows(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        );

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(Some(MatchResult { score, rect }))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
