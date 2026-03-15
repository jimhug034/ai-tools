//! 公共数据类型定义

use serde::{Deserialize, Serialize};

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: Option<String>,
    pub url: String,
}

/// 字幕轨道信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionTrack {
    pub base_url: String,
    pub language_code: String,
    pub name: Option<String>,
    pub kind: CaptionKind,
    pub is_translatable: bool,
}

/// 字幕类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptionKind {
    /// 用户上传的字幕
    Manual,
    /// YouTube 自动生成的字幕
    Auto,
}

/// 字幕格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubtitleFormat {
    /// SubRip 格式
    Srt,
    /// WebVTT 格式
    Vtt,
    /// Advanced SubStation Alpha 格式
    Ass,
    /// 纯文本格式
    Txt,
    /// JSON3 格式 (YouTube 原始)
    Json3,
    /// LRC 歌词格式
    Lrc,
}

impl SubtitleFormat {
    pub fn extension(&self) -> &str {
        match self {
            SubtitleFormat::Srt => "srt",
            SubtitleFormat::Vtt => "vtt",
            SubtitleFormat::Ass => "ass",
            SubtitleFormat::Txt => "txt",
            SubtitleFormat::Json3 => "json",
            SubtitleFormat::Lrc => "lrc",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            SubtitleFormat::Srt => "text/srt",
            SubtitleFormat::Vtt => "text/vtt",
            SubtitleFormat::Ass => "text/ass",
            SubtitleFormat::Txt => "text/plain",
            SubtitleFormat::Json3 => "application/json",
            SubtitleFormat::Lrc => "text/lrc",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "srt" => Some(SubtitleFormat::Srt),
            "vtt" | "webvtt" => Some(SubtitleFormat::Vtt),
            "ass" | "ssa" => Some(SubtitleFormat::Ass),
            "txt" => Some(SubtitleFormat::Txt),
            "json" | "json3" => Some(SubtitleFormat::Json3),
            "lrc" => Some(SubtitleFormat::Lrc),
            _ => None,
        }
    }
}

/// 字幕条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleEntry {
    pub text: String,
    pub start_ms: i64,
    pub duration_ms: i64,
}

impl SubtitleEntry {
    pub fn start_secs(&self) -> f64 {
        self.start_ms as f64 / 1000.0
    }

    pub fn duration_secs(&self) -> f64 {
        self.duration_ms as f64 / 1000.0
    }

    pub fn end_ms(&self) -> i64 {
        self.start_ms + self.duration_ms
    }

    pub fn end_secs(&self) -> f64 {
        self.end_ms() as f64 / 1000.0
    }
}

/// 字幕数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleData {
    pub entries: Vec<SubtitleEntry>,
}

/// YouTube Inner API 响应结构
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct YouTubePlayerResponse {
    #[serde(rename = "captions")]
    pub captions_data: Option<CaptionsData>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code, non_snake_case)]
pub(crate) struct CaptionsData {
    pub playerCaptionsTracklistRenderer: Option<CaptionsTracklistRenderer>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct CaptionsTracklistRenderer {
    pub caption_tracks: Option<Vec<CaptionTrackRaw>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct CaptionTrackRaw {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub name: Option<NameSimpleText>,
    #[serde(rename = "languageCode")]
    pub language_code: String,
    #[serde(rename = "kind", default)]
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct NameSimpleText {
    pub simple_text: String,
}

impl From<CaptionTrackRaw> for CaptionTrack {
    fn from(raw: CaptionTrackRaw) -> Self {
        let kind = match raw.kind.as_deref() {
            Some("asr") | Some("auto") => CaptionKind::Auto,
            _ => CaptionKind::Manual,
        };

        CaptionTrack {
            base_url: raw.base_url,
            language_code: raw.language_code,
            name: raw.name.map(|n| n.simple_text),
            kind,
            is_translatable: true, // YouTube 字幕通常可翻译
        }
    }
}

/// JSON3 格式字幕响应
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Json3Response {
    pub events: Vec<Json3Event>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Json3Event {
    #[serde(rename = "tStartMs")]
    pub start_ms: i64,
    #[serde(rename = "dDurationMs")]
    pub duration_ms: i64,
    pub segs: Option<Vec<Json3Segment>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Json3Segment {
    pub utf8: Option<String>,
}
