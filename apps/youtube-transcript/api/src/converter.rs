//! 字幕格式转换模块
//!
//! 参考 yt-dlp 实现，支持多种字幕格式之间的转换

use crate::error::Result;
use crate::types::{SubtitleData, SubtitleFormat};

/// 字幕格式转换器
pub struct SubtitleConverter;

impl SubtitleConverter {
    /// 将字幕数据转换为指定格式
    pub fn convert(data: &SubtitleData, format: SubtitleFormat) -> Result<String> {
        match format {
            SubtitleFormat::Srt => Self::to_srt(data),
            SubtitleFormat::Vtt => Self::to_vtt(data),
            SubtitleFormat::Ass => Self::to_ass(data),
            SubtitleFormat::Txt => Self::to_txt(data),
            SubtitleFormat::Json3 => Self::to_json3(data),
            SubtitleFormat::Lrc => Self::to_lrc(data),
        }
    }

    /// 转换为 SRT 格式
    ///
    /// SRT 格式示例:
    /// ```text
    /// 1
    /// 00:00:00,000 --> 00:00:03,000
    /// 第一条字幕
    ///
    /// 2
    /// 00:00:03,500 --> 00:00:07,000
    /// 第二条字幕
    /// ```
    pub fn to_srt(data: &SubtitleData) -> Result<String> {
        let mut output = String::new();

        for (i, entry) in data.entries.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }

            output.push_str(&format!("{}\n", i + 1));
            output.push_str(&format!(
                "{} --> {}\n",
                Self::format_srt_time(entry.start_ms),
                Self::format_srt_time(entry.end_ms())
            ));
            output.push_str(&entry.text);
            output.push('\n');
        }

        Ok(output)
    }

    /// 转换为 WebVTT 格式
    ///
    /// VTT 格式示例:
    /// ```text
    /// WEBVTT
    ///
    /// 00:00:00.000 --> 00:00:03.000
    /// 第一条字幕
    ///
    /// 00:00:03.500 --> 00:00:07.000
    /// 第二条字幕
    /// ```
    pub fn to_vtt(data: &SubtitleData) -> Result<String> {
        let mut output = String::from("WEBVTT\n\n");

        for entry in &data.entries {
            output.push_str(&format!(
                "{} --> {}\n",
                Self::format_vtt_time(entry.start_ms),
                Self::format_vtt_time(entry.end_ms())
            ));
            output.push_str(&entry.text);
            output.push('\n');
        }

        Ok(output)
    }

    /// 转换为 ASS 格式
    ///
    /// ASS (Advanced SubStation Alpha) 格式示例:
    /// ```text
    /// [Script Info]
    /// ScriptType: v4.00+
    /// WrapStyle: 0
    /// PlayResX: 1280
    /// PlayResY: 720
    ///
    /// [V4+ Styles]
    /// Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding
    /// Style: Default,Arial,16,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1
    ///
    /// [Events]
    /// Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
    /// Dialogue: 0,0:00:00.00,0:00:03.00,Default,,0,0,0,,第一条字幕
    /// ```
    pub fn to_ass(data: &SubtitleData) -> Result<String> {
        let mut output = String::from("[Script Info]\n");
        output.push_str("ScriptType: v4.00+\n");
        output.push_str("WrapStyle: 0\n");
        output.push_str("PlayResX: 1280\n");
        output.push_str("PlayResY: 720\n");
        output.push_str("\n");
        output.push_str("[V4+ Styles]\n");
        output.push_str("Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n");
        output.push_str("Style: Default,Arial,16,&H00FFFFFF,&H000000FF,&H00000000,&H00000000,0,0,0,0,100,100,0,0,1,2,0,2,10,10,10,1\n");
        output.push_str("\n");
        output.push_str("[Events]\n");
        output.push_str("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");

        for entry in &data.entries {
            // ASS 需要转义特殊字符
            let text = entry.text
                .replace('\\', "\\\\")
                .replace('{', "\\{")
                .replace('}', "\\}")
                .replace('\n', "\\N");

            output.push_str(&format!(
                "Dialogue: 0,{},{},Default,,0,0,0,,{}\n",
                Self::format_ass_time(entry.start_ms),
                Self::format_ass_time(entry.end_ms()),
                text
            ));
        }

        Ok(output)
    }

    /// 转换为纯文本格式
    pub fn to_txt(data: &SubtitleData) -> Result<String> {
        let texts: Vec<&str> = data.entries.iter()
            .map(|e| e.text.as_str())
            .collect();
        Ok(texts.join("\n\n"))
    }

    /// 转换为 JSON3 格式
    pub fn to_json3(data: &SubtitleData) -> Result<String> {
        let json_events: Vec<serde_json::Value> = data.entries.iter()
            .map(|entry| {
                serde_json::json!({
                    "tStartMs": entry.start_ms,
                    "dDurationMs": entry.duration_ms,
                    "segs": [{
                        "utf8": entry.text
                    }]
                })
            })
            .collect();

        let json = serde_json::json!({
            "events": json_events
        });

        serde_json::to_string_pretty(&json)
            .map_err(|e| crate::error::YtError::ConversionError(e.to_string()))
    }

    /// 转换为 LRC 格式
    ///
    /// LRC 格式示例:
    /// ```text
    /// [00:00.00]第一条字幕
    /// [00:03.50]第二条字幕
    /// ```
    pub fn to_lrc(data: &SubtitleData) -> Result<String> {
        let mut output = String::new();

        for (i, entry) in data.entries.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&format!(
                "[{}]{}",
                Self::format_lrc_time(entry.start_ms),
                entry.text
            ));
        }

        Ok(output)
    }

    /// 格式化 SRT 时间: 00:00:00,000
    fn format_srt_time(ms: i64) -> String {
        let hours = ms / 3600000;
        let minutes = (ms % 3600000) / 60000;
        let seconds = (ms % 60000) / 1000;
        let milliseconds = ms % 1000;
        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, milliseconds)
    }

    /// 格式化 VTT 时间: 00:00:00.000
    fn format_vtt_time(ms: i64) -> String {
        let hours = ms / 3600000;
        let minutes = (ms % 3600000) / 60000;
        let seconds = (ms % 60000) / 1000;
        let milliseconds = ms % 1000;
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
    }

    /// 格式化 ASS 时间: 0:00:00.00
    fn format_ass_time(ms: i64) -> String {
        let hours = ms / 3600000;
        let minutes = (ms % 3600000) / 60000;
        let seconds = (ms % 60000) as f64 / 1000.0;
        format!("{}:{:02}:{:05.2}", hours, minutes, seconds)
    }

    /// 格式化 LRC 时间: [00:00.00]
    fn format_lrc_time(ms: i64) -> String {
        let minutes = ms / 60000;
        let seconds = (ms % 60000) as f64 / 1000.0;
        format!("{:02}:{:05.2}", minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SubtitleEntry;

    #[test]
    fn test_to_srt() {
        let data = SubtitleData {
            entries: vec![
                SubtitleEntry {
                    text: "Hello World".to_string(),
                    start_ms: 0,
                    duration_ms: 3000,
                },
                SubtitleEntry {
                    text: "Second subtitle".to_string(),
                    start_ms: 3500,
                    duration_ms: 2000,
                },
            ],
        };

        let srt = SubtitleConverter::to_srt(&data).unwrap();
        assert!(srt.contains("00:00:00,000 --> 00:00:03,000"));
        assert!(srt.contains("Hello World"));
    }

    #[test]
    fn test_to_vtt() {
        let data = SubtitleData {
            entries: vec![
                SubtitleEntry {
                    text: "Hello World".to_string(),
                    start_ms: 0,
                    duration_ms: 3000,
                },
            ],
        };

        let vtt = SubtitleConverter::to_vtt(&data).unwrap();
        assert!(vtt.starts_with("WEBVTT"));
        assert!(vtt.contains("00:00:00.000 --> 00:00:03.000"));
    }

    #[test]
    fn test_to_lrc() {
        let data = SubtitleData {
            entries: vec![
                SubtitleEntry {
                    text: "First line".to_string(),
                    start_ms: 1500,
                    duration_ms: 3000,
                },
            ],
        };

        let lrc = SubtitleConverter::to_lrc(&data).unwrap();
        assert!(lrc.contains("[00:01.50]First line"));
    }
}
