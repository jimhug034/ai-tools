//! 字幕下载模块
//!
//! 下载字幕原始数据

use crate::error::{Result, YtError};
use crate::types::{SubtitleData, SubtitleEntry, CaptionTrack};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;

/// JSON3 格式字幕响应
#[derive(Debug, Deserialize)]
struct Json3Response {
    events: Vec<Json3Event>,
}

#[derive(Debug, Deserialize)]
struct Json3Event {
    #[serde(rename = "tStartMs")]
    start_ms: i64,
    #[serde(rename = "dDurationMs")]
    duration_ms: i64,
    segs: Option<Vec<Json3Segment>>,
}

#[derive(Debug, Deserialize)]
struct Json3Segment {
    utf8: Option<String>,
}

/// 字幕下载器
pub struct CaptionDownloader {
    client: Client,
}

impl CaptionDownloader {
    /// 创建新的下载器实例
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
        }
    }

    /// 使用自定义 HTTP 客户端创建下载器
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// 下载字幕并解析为统一格式
    pub async fn download(&self, track: &CaptionTrack) -> Result<SubtitleData> {
        // 添加 fmt=json3 参数获取 JSON 格式
        let json_url = if track.base_url.contains('?') {
            format!("{}&fmt=json3", track.base_url)
        } else {
            format!("{}?fmt=json3", track.base_url)
        };

        let response = self.client
            .get(&json_url)
            .send()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))?;

        // 先获取 content-type，避免借用问题
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let text = response
            .text()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))?;

        if content_type.contains("json") || text.trim().starts_with('{') {
            self.parse_json3(&text)
        } else {
            // 尝试解析 XML 格式（备用）
            self.parse_xml(&text)
        }
    }

    /// 解析 JSON3 格式字幕
    fn parse_json3(&self, content: &str) -> Result<SubtitleData> {
        let json: Json3Response = serde_json::from_str(content)
            .map_err(|e| YtError::ParseError(format!("JSON3 解析失败: {}", e)))?;

        let entries: Vec<SubtitleEntry> = json.events
            .into_iter()
            .filter_map(|event| self.parse_json3_event(event))
            .collect();

        if entries.is_empty() {
            return Err(YtError::ParseError("JSON3 中没有有效的字幕条目".to_string()));
        }

        Ok(SubtitleData { entries })
    }

    /// 解析单个 JSON3 事件
    fn parse_json3_event(&self, event: Json3Event) -> Option<SubtitleEntry> {
        let text = event.segs?
            .into_iter()
            .filter_map(|seg| seg.utf8)
            .collect::<String>();

        let text = clean_text(&text);
        if text.is_empty() {
            return None;
        }

        Some(SubtitleEntry {
            text,
            start_ms: event.start_ms,
            duration_ms: event.duration_ms,
        })
    }

    /// 解析 XML 格式字幕（备用方案）
    fn parse_xml(&self, content: &str) -> Result<SubtitleData> {
        let document = Html::parse_document(content);
        let selector = Selector::parse("transcript > text").unwrap();

        let entries: Vec<SubtitleEntry> = document
            .select(&selector)
            .filter_map(|element| {
                let text = element.text().collect::<String>();
                let text = clean_text(&text);
                if text.is_empty() {
                    return None;
                }

                let start = element.value().attr("start")
                    .and_then(|s| s.parse::<f64>().ok())?
                    * 1000.0;

                let dur = element.value().attr("dur")
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(3000.0); // 默认 3 秒
                let duration = (dur * 1000.0) as i64;

                Some(SubtitleEntry {
                    text,
                    start_ms: start as i64,
                    duration_ms: duration,
                })
            })
            .collect();

        if entries.is_empty() {
            return Err(YtError::ParseError("XML 中没有有效的字幕条目".to_string()));
        }

        Ok(SubtitleData { entries })
    }

    /// 下载原始文本（不解析）
    pub async fn download_raw(&self, track: &CaptionTrack) -> Result<String> {
        let response = self.client
            .get(&track.base_url)
            .send()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))?;

        response
            .text()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))
    }
}

impl Default for CaptionDownloader {
    fn default() -> Self {
        Self::new()
    }
}

/// 清理字幕文本
fn clean_text(text: &str) -> String {
    let text = decode_html_entities(text);
    // 移除 HTML 标签
    let text = regex::Regex::new(r"<[^>]+>")
        .unwrap()
        .replace_all(&text, "");
    // 移除多余空格
    let text = regex::Regex::new(r"\s+")
        .unwrap()
        .replace_all(&text, " ");
    text.trim().to_string()
}

/// HTML 实体解码
fn decode_html_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("<br>", "")
        .replace("<br/>", "")
        .replace("<br />", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        assert_eq!(clean_text("Hello World"), "Hello World");
        assert_eq!(clean_text("Hello  &amp;  World"), "Hello & World");
        assert_eq!(clean_text("Hello<br>World"), "HelloWorld");
        assert_eq!(clean_text("  Hello   World  "), "Hello World");
    }
}
