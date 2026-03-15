//! YouTube Innertube API 客户端
//!
//! 参考 yt-dlp 实现，使用 YouTube 的 innertube API 获取字幕信息

use crate::error::{Result, YtError};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

/// API key 正则表达式（预编译，避免重复编译）
static API_KEY_PATTERNS: Lazy<[Regex; 3]> = Lazy::new(|| [
    Regex::new(r#""INNERTUBE_API_KEY"\s*:\s*"([^"]+)""#).unwrap(),
    Regex::new(r#"innertubeApiKey"\s*:\s*"([^"]+)""#).unwrap(),
    Regex::new(r#"apiKey"\s*:\s*"([^"]+)""#).unwrap(),
]);

/// 默认 innertube API key
const DEFAULT_API_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

/// Innertube API 客户端
pub struct InnertubeClient {
    client: Client,
}

impl InnertubeClient {
    /// 创建新的 innertube 客户端
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
                .build()
                .unwrap_or_default(),
        }
    }

    /// 使用自定义客户端创建
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// 从 HTML 页面提取 innertube API key
    pub fn extract_api_key(&self, html: &str) -> Result<String> {
        // 尝试从多个位置提取 API key（使用预编译的正则）
        for re in API_KEY_PATTERNS.iter() {
            if let Some(caps) = re.captures(html) {
                return Ok(caps[1].to_string());
            }
        }

        // 使用默认的 innertube API key
        Ok(DEFAULT_API_KEY.to_string())
    }

    /// 获取播放器信息（包含字幕信息）
    pub async fn get_player_info(&self, video_id: &str, html: &str) -> Result<PlayerResponse> {
        // 首先从页面提取 API key
        let api_key = self.extract_api_key(html)?;

        // 参考 yt-dlp 的客户端配置，按优先级尝试
        // ANDROID_VR 的字幕 URL 通常不需要 PO Token
        let client_configs = vec![
            // Android VR - 优先尝试，通常返回无 exp=xpe 的 URL
            ClientConfig {
                client_name: "ANDROID_VR",
                client_version: "1.65.10",
                client_name_header: Some("28"),
                device_model: Some("Quest 3"),
                os_name: Some("Android"),
                os_version: Some("12L"),
                user_agent: Some("com.google.android.apps.youtube.vr.oculus/1.65.10 (Linux; U; Android 12L; eureka-user Build/SQ3A.220605.009.A1) gzip"),
            },
            // Android - 备选
            ClientConfig {
                client_name: "ANDROID",
                client_version: "19.09.37",
                client_name_header: Some("3"),
                device_model: Some("samsung SM-G998B"),
                os_name: Some("Android"),
                os_version: Some("11"),
                user_agent: Some("com.google.android.youtube/19.09.37 (Linux; U; Android 11) gzip"),
            },
            // iOS - 备选
            ClientConfig {
                client_name: "IOS",
                client_version: "21.02.3",
                client_name_header: Some("5"),
                device_model: Some("iPhone16,2"),
                os_name: Some("iPhone"),
                os_version: Some("18.3.2.22D82"),
                user_agent: Some("com.google.ios.youtube/21.02.3 (iPhone16,2; U; CPU iOS 18_3_2 like Mac OS X;)"),
            },
            // MWEB - 移动网站
            ClientConfig {
                client_name: "MWEB",
                client_version: "2.20260115.01.00",
                client_name_header: Some("2"),
                device_model: None,
                os_name: None,
                os_version: None,
                user_agent: Some("Mozilla/5.0 (iPad; CPU OS 16_7_10 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1,gzip(gfe)"),
            },
        ];

        for config in client_configs {
            eprintln!("Trying innertube with client: {}", config.client_name);

            // 构建请求体
            let body = InnertubeRequestBody {
                context: Context {
                    client: ClientInfo {
                        client_name: config.client_name.to_string(),
                        client_version: config.client_version.to_string(),
                        device_model: config.device_model.map(|s| s.to_string()),
                        os_name: config.os_name.map(|s| s.to_string()),
                        os_version: config.os_version.map(|s| s.to_string()),
                        user_agent: config.user_agent.map(|s| s.to_string()),
                        hl: "en".to_string(),
                        gl: "US".to_string(),
                    },
                },
                video_id: video_id.to_string(),
            };

            let url = format!(
                "https://www.youtube.com/youtubei/v1/player?key={}",
                api_key
            );

            let mut request_builder = self
                .client
                .post(&url)
                .header("Content-Type", "application/json");

            // 添加 X-YouTube-Client-Name 头（如果有的话）
            if let Some(client_name_header) = config.client_name_header {
                request_builder = request_builder.header("X-YouTube-Client-Name", client_name_header);
            }

            let response = request_builder
                .json(&body)
                .send()
                .await
                .map_err(|e| YtError::FetchFailed(format!("Innertube API request failed: {}", e)))?;

            let status = response.status();
            let text = response
                .text()
                .await
                .map_err(|e| YtError::FetchFailed(format!("Failed to read response: {}", e)))?;

            if !status.is_success() {
                eprintln!("Innertube API error with {}: status={}", config.client_name, status);
                continue;
            }

            eprintln!("Innertube API response length with {}: {}", config.client_name, text.len());

            // 尝试解析
            match self.parse_player_response(&text) {
                Ok(response) => {
                    eprintln!("Successfully parsed response with client: {}", config.client_name);
                    return Ok(response);
                }
                Err(e) => {
                    eprintln!("Failed to parse with {}: {:?}", config.client_name, e);
                    // 继续尝试下一个客户端
                }
            }
        }

        Err(YtError::NoCaptionsFound)
    }

    /// 解析播放器响应
    fn parse_player_response(&self, text: &str) -> Result<PlayerResponse> {
        eprintln!("Parsing player response, length: {}", text.len());

        let json: Value = serde_json::from_str(text)
            .map_err(|e| YtError::ParseError(format!("Failed to parse player response: {}", e)))?;

        // 打印顶层键，用于调试
        if let Some(obj) = json.as_object() {
            let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            eprintln!("JSON top-level keys: {:?}", keys);
        }

        // 检查是否有错误
        if let Some(error) = json.get("error") {
            eprintln!("Innertube API returned error: {}", error);
            return Err(YtError::FetchFailed(format!(
                "Innertube API returned error: {}",
                error
            )));
        }

        // 提取字幕信息 - 尝试多个可能的路径
        let caption_paths = [
            "/captions/playerCaptionsTracklistRenderer",
            "/playerCaptionsTracklistRenderer",
        ];

        for path in &caption_paths {
            if let Some(captions) = json.pointer(path) {
                eprintln!("Found captions at path: {}", path);
                if let Some(obj) = captions.as_object() {
                    eprintln!("Captions keys: {:?}", obj.keys().collect::<Vec<_>>());
                }

                return Ok(PlayerResponse {
                    captions: captions.clone(),
                    raw_json: json,
                });
            }
        }

        // 检查 playabilityStatus - 视频可能不可用
        if let Some(playability) = json.get("playabilityStatus") {
            if let Some(status) = playability.get("status").and_then(|s| s.as_str()) {
                if status != "OK" {
                    eprintln!("Video playability status: {}", status);
                    if let Some(reason) = playability.get("reason").and_then(|r| r.as_str()) {
                        eprintln!("Reason: {}", reason);
                    }
                }
            }
        }

        eprintln!("Captions not found at any expected path");
        Err(YtError::NoCaptionsFound)
    }

    /// 从播放器响应中提取字幕轨道
    pub fn extract_caption_tracks(&self, player_response: &PlayerResponse) -> Vec<CaptionTrackInfo> {
        let mut tracks = Vec::new();

        if let Some(renderer) = player_response.captions.get("captionTracks") {
            if let Some(track_array) = renderer.as_array() {
                for track in track_array {
                    if let Ok(info) = serde_json::from_value::<CaptionTrackInfo>(track.clone()) {
                        eprintln!("Parsed track: {} ({})", info.language_code, info.name.as_ref().map(|n| n.get_text()).unwrap_or_default());
                        tracks.push(info);
                    }
                }
            }
        }

        tracks
    }
}

impl Default for InnertubeClient {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== 客户端配置 ====================

struct ClientConfig<'a> {
    client_name: &'a str,
    client_version: &'a str,
    client_name_header: Option<&'a str>,
    device_model: Option<&'a str>,
    os_name: Option<&'a str>,
    os_version: Option<&'a str>,
    user_agent: Option<&'a str>,
}

// ==================== 请求体结构 ====================

#[derive(Debug, Serialize)]
struct InnertubeRequestBody {
    context: Context,
    video_id: String,
}

#[derive(Debug, Serialize)]
struct Context {
    client: ClientInfo,
}

#[derive(Debug, Serialize)]
struct ClientInfo {
    client_name: String,
    client_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,
    hl: String,
    gl: String,
}

// ==================== 响应结构 ====================

#[derive(Debug, Clone)]
pub struct PlayerResponse {
    pub captions: serde_json::Value,
    pub raw_json: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CaptionTrackInfo {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "languageCode")]
    pub language_code: String,
    pub name: Option<NameText>,
    pub kind: Option<String>,
    #[serde(rename = "isTranslatable")]
    pub is_translatable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NameText {
    Simple(SimpleNameText),
    Runs(RunsNameText),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleNameText {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RunsNameText {
    pub runs: Vec<NameRun>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NameRun {
    pub text: String,
}

impl NameText {
    /// 获取文本内容
    pub fn get_text(&self) -> String {
        match self {
            NameText::Simple(s) => s.simple_text.clone(),
            NameText::Runs(r) => {
                // 预计算容量以避免重新分配
                let total_len: usize = r.runs.iter().map(|run| run.text.len()).sum();
                let mut result = String::with_capacity(total_len);
                for (i, run) in r.runs.iter().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&run.text);
                }
                result
            }
        }
    }

    /// 获取文本内容的引用（避免分配）
    pub fn as_text(&self) -> Cow<'_, str> {
        match self {
            NameText::Simple(s) => Cow::Borrowed(&s.simple_text),
            NameText::Runs(_) => Cow::Owned(self.get_text()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_api_key() {
        let html = r#"{"INNERTUBE_API_KEY":"test_key"}"#;
        let client = InnertubeClient::new();
        assert_eq!(client.extract_api_key(html).unwrap(), "test_key");
    }
}
