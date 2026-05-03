use crate::gamebot::error::BotError;
use crate::gamebot::template::{MatchResult, UiTemplate};
use opencv::{core, imgcodecs, imgproc, prelude::*};
use std::collections::HashMap;
use std::path::Path;

pub struct TemplateDetector {
    cache: HashMap<String, Mat>,
}

impl TemplateDetector {
    fn cache_key(t: &UiTemplate) -> String {
        let normalized_path = Path::new(&t.image_path)
            .canonicalize()
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .unwrap_or_else(|| t.image_path.clone());
        format!("{}::{}", t.name, normalized_path)
    }

    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn ensure_loaded(&mut self, t: &UiTemplate) -> Result<(), BotError> {
        let cache_key = Self::cache_key(t);
        if self.cache.contains_key(&cache_key) {
            return Ok(());
        }
        let mat = imgcodecs::imread(&t.image_path, imgcodecs::IMREAD_COLOR)
            .map_err(|e| BotError::Vision(format!("模板读取失败 {}: {e}", t.image_path)))?;
        if mat.empty() {
            return Err(BotError::Vision(format!(
                "模板为空或路径错误: {}",
                t.image_path
            )));
        }
        self.cache.insert(cache_key, mat);
        Ok(())
    }

    pub fn locate(&mut self, frame: &Mat, t: &UiTemplate) -> Result<Option<MatchResult>, BotError> {
        self.ensure_loaded(t)?;
        let cache_key = Self::cache_key(t);
        let tpl = self
            .cache
            .get(&cache_key)
            .ok_or_else(|| BotError::Vision(format!("模板未加载: {}", t.image_path)))?;

        let roi = if let Some(r) = t.search_region {
            Mat::roi(frame, r).map_err(|e| BotError::Vision(format!("ROI 裁剪失败: {e}")))?
        } else {
            frame.clone().into()
        };

        let result_cols = roi.cols() - tpl.cols() + 1;
        let result_rows = roi.rows() - tpl.rows() + 1;
        if result_cols <= 0 || result_rows <= 0 {
            return Ok(None);
        }

        let mut result = Mat::zeros(result_rows, result_cols, core::CV_32FC1)
            .map_err(|e| BotError::Vision(format!("创建结果矩阵失败: {e}")))?
            .to_mat()
            .map_err(|e| BotError::Vision(format!("to_mat 失败: {e}")))?;

        imgproc::match_template(
            &roi,
            tpl,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )
        .map_err(|e| BotError::Vision(format!("match_template 失败: {e}")))?;

        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = core::Point::default();
        let mut max_loc = core::Point::default();

        core::min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &Mat::default(),
        )
        .map_err(|e| BotError::Vision(format!("min_max_loc 失败: {e}")))?;

        let score = max_val as f32;
        if score < t.threshold {
            return Ok(None);
        }

        let base = t.search_region.unwrap_or(core::Rect::new(0, 0, 0, 0));
        let rect = core::Rect::new(
            max_loc.x + base.x,
            max_loc.y + base.y,
            tpl.cols(),
            tpl.rows(),
        );

        Ok(Some(MatchResult { score, rect }))
    }
}
