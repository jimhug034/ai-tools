//! 字幕下载模块
//!
//! 下载字幕原始数据
//! 使用 innertube API 获取的字幕 URL（不包含 exp=xpe，不需要 PO Token）

use crate::error::{Result, YtError};
use crate::types::{SubtitleData, SubtitleEntry, CaptionTrack};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;
use regex::Regex;

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
///
/// 使用普通 HTTP 客户端下载字幕
/// innertube API 提供的字幕 URL 应该不需要 PO Token
pub struct CaptionDownloader {
    /// HTTP 客户端
    client: Client,
}

impl CaptionDownloader {
    /// 创建新的下载器实例
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .build()
            .unwrap_or_default();

        Ok(Self { client })
    }

    /// 使用自定义 HTTP 客户端创建下载器
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// 下载字幕并解析为统一格式
    pub async fn download(&self, track: &CaptionTrack) -> Result<SubtitleData> {
        eprintln!("Downloading caption from: {}", track.base_url);

        let response = self.client
            .get(&track.base_url)
            .header("Cookie", "CONSENT=YES+cb")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept", "*/*")
            .header("Referer", "https://www.youtube.com/")
            .send()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))?;

        let status = response.status();
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

        eprintln!("Response - status: {}, content-type: {}, length: {}",
                  status, content_type, text.len());

        // 检查是否是有效的响应
        if text.is_empty() {
            return Err(YtError::DownloadFailed("字幕内容为空".to_string()));
        }

        if content_type.contains("html") || text.contains("<!DOCTYPE") || text.contains("<html") {
            eprintln!("HTML response, trying to rebuild URL");
            return self.download_with_rebuilt_url(track).await;
        }

        // 优先尝试解析 XML 格式（YouTube 默认格式，更稳定）
        if text.contains("<transcript>") || text.contains("<p ") || text.contains("<text ") || text.trim().starts_with('<') {
            eprintln!("Parsing XML format caption");
            self.parse_xml(&text)
        } else if content_type.contains("json") || text.trim().starts_with('{') {
            eprintln!("Parsing JSON3 format caption");
            self.parse_json3(&text)
        } else {
            eprintln!("Unknown format, trying XML");
            self.parse_xml(&text)
        }
    }

    /// 重建 URL 并下载（当原 URL 过期时）
    async fn download_with_rebuilt_url(&self, track: &CaptionTrack) -> Result<SubtitleData> {
        use regex::Regex;

        // 从 base_url 中提取视频 ID
        let url = &track.base_url;

        // 提取 v 参数
        let v_re = Regex::new(r"[?&]v=([^&]+)").unwrap();
        let video_id = v_re.captures(url)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .ok_or_else(|| YtError::DownloadFailed("无法提取视频 ID".to_string()))?;

        // 尝试多种 URL 格式
        let kind_param = if track.kind == crate::types::CaptionKind::Auto {
            "&kind=asr"
        } else {
            ""
        };

        let urls = vec![
            // 格式 1: 基本格式
            format!(
                "https://www.youtube.com/api/timedtext?v={}&lang={}&fmt=json3{}",
                video_id, track.language_code, kind_param
            ),
            // 格式 2: 添加 expire 参数
            format!(
                "https://www.youtube.com/api/timedtext?v={}&lang={}&fmt=json3{}&expire={}",
                video_id, track.language_code, kind_param,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 3600
            ),
            // 格式 3: 使用 name 参数
            format!(
                "https://www.youtube.com/api/timedtext?v={}&lang={}&name={}&fmt=json3{}",
                video_id, track.language_code,
                urlencoding::encode(track.name.as_deref().unwrap_or("en")),
                kind_param
            ),
        ];

        for new_url in urls {
            eprintln!("Trying rebuilt URL: {}", new_url);

            let response = self.client
                .get(&new_url)
                .header("Cookie", "CONSENT=YES+cb; PREF=f6=40000000")
                .header("Accept-Language", "en-US,en;q=0.9")
                .header("Accept", "*/*")
                .header("Referer", "https://www.youtube.com/")
                .header("Origin", "https://www.youtube.com")
                .header("Sec-Fetch-Dest", "empty")
                .header("Sec-Fetch-Mode", "cors")
                .header("Sec-Fetch-Site", "same-site")
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
                .send()
                .await;

            if let Err(e) = response {
                eprintln!("Request failed: {}", e);
                continue;
            }

            let response = response.unwrap();
            let status = response.status();
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let text = response.text().await.unwrap_or_default();

            eprintln!("Response - status: {}, content-type: {}, length: {}",
                      status, content_type, text.len());

            // 检查是否是有效的响应
            if text.is_empty() || text.contains("<!DOCTYPE") || text.contains("<html") || content_type.contains("html") {
                eprintln!("Invalid response, trying next URL format");
                continue;
            }

            // 如果是 JSON，尝试解析
            if text.trim().starts_with('{') {
                eprintln!("Got JSON response, parsing...");
                return self.parse_json3(&text);
            }

            // 尝试 XML 解析
            if text.contains("<transcript>") || text.contains("<tt ") {
                eprintln!("Got XML response, parsing...");
                return self.parse_xml(&text);
            }

            eprintln!("Unknown response format, trying next URL format");
        }

        Err(YtError::DownloadFailed("所有 URL 格式都无法获取字幕".to_string()))
    }

    /// 解析 JSON3 格式字幕
    fn parse_json3(&self, content: &str) -> Result<SubtitleData> {
        eprintln!("Parsing JSON3, content preview: {}...", &content[..content.len().min(200)]);

        let json: Json3Response = serde_json::from_str(content)
            .map_err(|e| {
                eprintln!("JSON3 parse error: {}", e);
                eprintln!("Content was: {}", &content[..content.len().min(500)]);
                YtError::ParseError(format!("JSON3 解析失败: {}", e))
            })?;

        eprintln!("Found {} events in JSON3", json.events.len());

        let entries: Vec<SubtitleEntry> = json.events
            .into_iter()
            .filter_map(|event| self.parse_json3_event(event))
            .collect();

        eprintln!("Parsed {} valid subtitle entries", entries.len());

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
        // 首先尝试解析 <p t="..." d="..."> 格式（youtubei.js 使用的格式）
        if content.contains("<p ") {
            eprintln!("Trying <p> tag format parsing");
            let p_entries = self.parse_p_tag_format(content);
            if !p_entries.is_empty() {
                return Ok(SubtitleData { entries: p_entries });
            }
        }

        // 然后尝试解析 <text start="..." dur="..."> 格式
        if content.contains("<text ") {
            eprintln!("Trying <text> tag format parsing");
            let document = Html::parse_document(content);
            let selector = Selector::parse("text").unwrap();

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

            if !entries.is_empty() {
                return Ok(SubtitleData { entries });
            }
        }

        Err(YtError::ParseError("XML 中没有有效的字幕条目".to_string()))
    }

    /// 解析 <p t="..." d="..."> 格式的字幕（youtubei.js 格式）
    fn parse_p_tag_format(&self, content: &str) -> Vec<SubtitleEntry> {
        use regex::Regex;

        // 匹配 <p t="开始时间(ms)" d="持续时间(ms)">文本</p>
        let p_re = Regex::new(r#"<p\s+t="(\d+)"\s+d="(\d+)"[^>]*>([\s\S]*?)</p>"#).unwrap();

        p_re.captures_iter(content)
            .filter_map(|caps| {
                let start_ms: i64 = caps.get(1).and_then(|m| m.as_str().parse().ok())?;
                let duration_ms: i64 = caps.get(2).and_then(|m| m.as_str().parse().ok())?;
                let raw_text = caps.get(3).map(|m| m.as_str()).unwrap_or("");

                let text = clean_text(raw_text);
                if text.is_empty() {
                    return None;
                }

                Some(SubtitleEntry {
                    text,
                    start_ms,
                    duration_ms,
                })
            })
            .collect()
    }

    /// 从 URL 直接下载并解析字幕
    pub async fn download_from_url(&self, url: &str) -> Result<SubtitleData> {
        // URL 可能已经包含了 fmt 参数，需要检查
        let json_url = if url.contains("fmt=") {
            url.to_string()
        } else if url.contains('?') {
            format!("{}&fmt=json3", url)
        } else {
            format!("{}?fmt=json3", url)
        };

        eprintln!("Downloading from URL: {}", json_url);

        let response = self.client
            .get(&json_url)
            .header("Cookie", "CONSENT=YES+cb; PREF=f6=40000000; VISITOR_INFO1_LIVE=; YSC=")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept", "*/*")
            .header("Referer", "https://www.youtube.com/")
            .header("Origin", "https://www.youtube.com")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-site")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(|e| YtError::DownloadFailed(e.to_string()))?;

        let status = response.status();
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

        eprintln!("Response - status: {}, content-type: {}, length: {}",
                  status, content_type, text.len());

        // 如果响应是 HTML 或空，说明请求被拒绝
        if text.is_empty() || text.contains("<!DOCTYPE") || text.contains("<html") || content_type.contains("html") {
            return Err(YtError::DownloadFailed(format!(
                "字幕请求被拒绝 (status: {}, content-type: {})",
                status, content_type
            )));
        }

        if content_type.contains("json") || text.trim().starts_with('{') {
            self.parse_json3(&text)
        } else {
            eprintln!("Trying XML fallback parsing");
            self.parse_xml(&text)
        }
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
        Self::new().expect("Failed to create default CaptionDownloader")
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

/// 从 YouTube 页面 HTML 中提取内嵌的字幕数据
/// 这作为备用方案，当 API 请求被阻止时使用
pub async fn extract_embedded_captions(html: &str, _language_code: &str) -> Result<SubtitleData> {
    // 查找页面中内嵌的字幕 JSON 数据
    // YouTube 有时会在页面中包含字幕的转义 JSON

    // 尝试查找 captionTracks 数据中的内嵌内容
    let caption_re = Regex::new(r#""captionTracks"\s*:\s*\[([^\]]+)\]"#).unwrap();
    if let Some(_caps) = caption_re.captures(html) {
        eprintln!("Found captionTracks in HTML");
        // 这个数据可能包含 baseUrl，但我们已经尝试过了
    }

    // 尝试查找 "captions" JSON 数据
    let captions_re = Regex::new(r#""captions"\s*:\s*\{[^}]*"playerCaptionsTracklistRenderer"\s*:\s*\{[^}]*"captionTracks"[^}]*\}"#).unwrap();
    if let Some(_caps) = captions_re.captures(html) {
        eprintln!("Found captions data in HTML");
    }

    Err(YtError::DownloadFailed("无法从页面中提取内嵌字幕".to_string()))
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
