//! 字幕信息提取模块
//!
//! 从 YouTube 页面提取可用的字幕信息

use crate::error::{Result, YtError};
use crate::types::{CaptionTrack, VideoInfo};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;

/// 从 YouTube URL 提取视频 ID
pub fn extract_video_id(url: &str) -> Result<String> {
    // 短链接格式: youtu.be/VIDEO_ID 或 youtube.com/embed/VIDEO_ID
    static SHORT_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?:(?:https?://)?(?:www\.)?youtu\.be/|youtube\.com/embed/)([a-zA-Z0-9_-]{11})").unwrap()
    });

    // 长链接格式: ?v=VIDEO_ID
    static LONG_PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[?&]v=([a-zA-Z0-9_-]{11})").unwrap()
    });

    // 先尝试短链接格式
    if let Some(caps) = SHORT_PATTERN.captures(url) {
        return Ok(caps[1].to_string());
    }

    // 尝试长链接格式
    if let Some(caps) = LONG_PATTERN.captures(url) {
        return Ok(caps[1].to_string());
    }

    // 直接输入的 VIDEO_ID
    if url.len() == 11 && url.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Ok(url.to_string());
    }

    Err(YtError::VideoIdExtractionFailed)
}

/// 字幕信息提取器
pub struct CaptionExtractor {
    client: Client,
}

impl CaptionExtractor {
    /// 创建新的提取器实例
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
        }
    }

    /// 使用自定义 HTTP 客户端创建提取器
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// 提取视频信息
    pub async fn extract_video_info(&self, url: &str) -> Result<VideoInfo> {
        let video_id = extract_video_id(url)?;
        let watch_url = format!("https://www.youtube.com/watch?v={}", video_id);

        let html = self.client
            .get(&watch_url)
            .send()
            .await
            .map_err(|e| YtError::FetchFailed(e.to_string()))?
            .text()
            .await
            .map_err(|e| YtError::FetchFailed(e.to_string()))?;

        // 提取视频标题
        let title = self.extract_title(&html);

        Ok(VideoInfo {
            id: video_id,
            title,
            url: watch_url,
        })
    }

    /// 提取可用的字幕轨道列表
    pub async fn extract_caption_tracks(&self, video_id: &str) -> Result<Vec<CaptionTrack>> {
        let url = format!("https://www.youtube.com/watch?v={}", video_id);

        let html = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| YtError::FetchFailed(e.to_string()))?
            .text()
            .await
            .map_err(|e| YtError::FetchFailed(e.to_string()))?;

        self.parse_caption_tracks(&html)
    }

    /// 从 HTML 中解析字幕轨道
    fn parse_caption_tracks(&self, html: &str) -> Result<Vec<CaptionTrack>> {
        // 方法 1: 解析 ytInitialData JSON
        if let Some(data) = extract_yt_initial_data(html) {
            if let Ok(tracks) = parse_caption_tracks_from_json(&data) {
                if !tracks.is_empty() {
                    return Ok(tracks);
                }
            }
        }

        // 方法 2: 使用正则表达式直接搜索
        self.extract_caption_tracks_by_regex(html)
    }

    /// 提取视频标题
    fn extract_title(&self, html: &str) -> Option<String> {
        // 尝试从 meta 标签提取
        static META_TITLE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"<meta\s+name="title"\s+content="([^"]+)""#).unwrap()
        });

        if let Some(caps) = META_TITLE.captures(html) {
            return Some(caps[1].to_string());
        }

        // 尝试从 ytInitialData 提取
        if let Some(data) = extract_yt_initial_data(html) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(title) = json.pointer("/videoDetails/title") {
                    if let Some(s) = title.as_str() {
                        return Some(s.to_string());
                    }
                }
            }
        }

        // 尝试从 <title> 标签提取
        static TITLE_TAG: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"<title>([^<]+)</title>").unwrap()
        });

        TITLE_TAG.captures(html).map(|caps| {
            // YouTube 标题格式: "视频标题 - YouTube"
            caps[1].split(" - YouTube").next().unwrap_or(&caps[1]).to_string()
        })
    }

    /// 使用正则表达式提取字幕轨道
    fn extract_caption_tracks_by_regex(&self, html: &str) -> Result<Vec<CaptionTrack>> {
        let mut tracks = Vec::new();

        // 搜索所有 "baseUrl" 配合 "languageCode"
        static BASE_URL_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#""baseUrl"\s*:\s*"((?:[^"\\]|\\.)+)""#).unwrap()
        });

        static LANG_CODE_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#""languageCode"\s*:\s*"([^"]+)""#).unwrap()
        });

        static NAME_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#""name"\s*:\s*\{\s*"simpleText"\s*:\s*"((?:[^"\\]|\\.)+)""#).unwrap()
        });

        static KIND_RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#""kind"\s*:\s*"([^"]+)""#).unwrap()
        });

        let base_urls: Vec<String> = BASE_URL_RE.captures_iter(html)
            .filter_map(|caps| caps.get(1))
            .map(|m| m.as_str().replace("\\u0026", "&").replace("\\", ""))
            .filter(|s| s.contains("timedtext"))
            .collect();

        let lang_codes: Vec<String> = LANG_CODE_RE.captures_iter(html)
            .filter_map(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .collect();

        let names: Vec<String> = NAME_RE.captures_iter(html)
            .filter_map(|caps| caps.get(1))
            .map(|m| m.as_str().replace("\\u0026", "&").replace("\\", ""))
            .collect();

        let kinds: Vec<String> = KIND_RE.captures_iter(html)
            .filter_map(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .collect();

        // 配对数据 - 简化版本，假设顺序一致
        let count = base_urls.len().min(lang_codes.len());
        for i in 0..count {
            let base_url = &base_urls[i];
            let lang_code = &lang_codes[i];
            let name = names.get(i).cloned();
            let kind = kinds.get(i).cloned();

            tracks.push(CaptionTrack {
                base_url: base_url.clone(),
                language_code: lang_code.clone(),
                name,
                kind: match kind.as_deref() {
                    Some("asr") | Some("auto") => crate::types::CaptionKind::Auto,
                    _ => crate::types::CaptionKind::Manual,
                },
                is_translatable: true,
            });
        }

        if tracks.is_empty() {
            Err(YtError::NoCaptionsFound)
        } else {
            Ok(tracks)
        }
    }
}

impl Default for CaptionExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// 从 HTML 中提取 ytInitialData
fn extract_yt_initial_data(html: &str) -> Option<String> {
    // 搜索 ytInitialData = {...}
    static INITIAL_DATA_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"ytInitialData\s*=\s*(\{.+?\});").unwrap()
    });

    if let Some(caps) = INITIAL_DATA_RE.captures(html) {
        return Some(caps[1].to_string());
    }

    // 备用模式：var ytInitialData = {...}
    static VAR_INITIAL_DATA_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"var\s+ytInitialData\s*=\s*(\{.+?\});").unwrap()
    });

    VAR_INITIAL_DATA_RE.captures(html)
        .map(|caps| caps[1].to_string())
}

/// 从 JSON 数据中解析字幕轨道
fn parse_caption_tracks_from_json(data: &str) -> Result<Vec<CaptionTrack>> {
    let json: serde_json::Value = serde_json::from_str(data)
        .map_err(|e| YtError::ParseError(e.to_string()))?;

    let mut results = Vec::new();

    // 递归查找 captionTracks
    find_caption_tracks_in_value(&json, &mut results);

    if results.is_empty() {
        Err(YtError::NoCaptionsFound)
    } else {
        Ok(results)
    }
}

/// 递归查找 caption_tracks
fn find_caption_tracks_in_value(
    value: &serde_json::Value,
    results: &mut Vec<CaptionTrack>,
) {
    use crate::types::CaptionTrackRaw;

    // 检查 captionTracks 字段
    if let Some(caption_tracks) = value.get("captionTracks") {
        if let Some(tracks) = caption_tracks.as_array() {
            for track in tracks {
                if let Ok(raw) = serde_json::from_value::<CaptionTrackRaw>(track.clone()) {
                    results.push(raw.into());
                }
            }
        }
    }

    // 检查 captions.playerCaptionsTracklistRenderer.captionTracks
    if let Some(captions) = value.get("captions") {
        if let Some(renderer) = captions.get("playerCaptionsTracklistRenderer") {
            if let Some(tracks) = renderer.get("captionTracks") {
                if let Some(tracks_array) = tracks.as_array() {
                    for track in tracks_array {
                        if let Ok(raw) = serde_json::from_value::<CaptionTrackRaw>(track.clone()) {
                            results.push(raw.into());
                        }
                    }
                }
            }
        }
    }

    // 递归搜索子对象
    if let Some(obj) = value.as_object() {
        for val in obj.values() {
            find_caption_tracks_in_value(val, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap(),
            "dQw4w9WgXcQ"
        );
        assert_eq!(
            extract_video_id("https://youtu.be/dQw4w9WgXcQ").unwrap(),
            "dQw4w9WgXcQ"
        );
        assert_eq!(
            extract_video_id("https://www.youtube.com/embed/dQw4w9WgXcQ").unwrap(),
            "dQw4w9WgXcQ"
        );
        assert_eq!(
            extract_video_id("dQw4w9WgXcQ").unwrap(),
            "dQw4w9WgXcQ"
        );
    }
}
