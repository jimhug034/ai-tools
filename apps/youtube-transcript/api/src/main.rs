use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Debug, Deserialize)]
struct TranscriptRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct TranscriptItem {
    text: String,
    #[serde(rename = "duration")]
    duration_secs: f64,
    #[serde(rename = "offset")]
    offset_secs: f64,
}

#[derive(Debug, Serialize)]
struct TranscriptResult {
    #[serde(rename = "videoId")]
    video_id: String,
    items: Vec<TranscriptItem>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

// HTML 实体解码
fn decode_html_entities(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

// 从 YouTube URL 提取 video ID
fn extract_video_id(url: &str) -> Option<String> {
    // youtu.be/VIDEO_ID
    if url.contains("youtu.be/") {
        if let Some(idx) = url.find("youtu.be/") {
            let rest = &url[idx + 9..];
            let id = rest.split('?').next().unwrap_or(rest);
            return Some(id.trim().to_string());
        }
    }

    // youtube.com/watch?v=VIDEO_ID
    if url.contains("v=") {
        if let Some(idx) = url.find("v=") {
            let rest = &url[idx + 2..];
            let id = rest.split('&').next().unwrap_or(rest);
            return Some(id.trim().to_string());
        }
    }

    // youtube.com/embed/VIDEO_ID
    if url.contains("/embed/") {
        if let Some(idx) = url.find("/embed/") {
            let rest = &url[idx + 7..];
            let id = rest.split('?').next().unwrap_or(rest);
            return Some(id.trim().to_string());
        }
    }

    None
}

// 使用 yt-dlp 获取字幕
async fn get_transcript_via_ytdlp(video_id: &str) -> Result<TranscriptResult, String> {
    let mut cmd = Command::new("yt-dlp");

    // 设置代理
    if let Ok(proxy) = std::env::var("HTTPS_PROXY") {
        cmd.env("HTTPS_PROXY", proxy);
    }
    if let Ok(proxy) = std::env::var("HTTP_PROXY") {
        cmd.env("HTTP_PROXY", proxy);
    }

    // yt-dlp 参数：下载字幕为 JSON3 格式
    // 使用 android 客户端绕过 YouTube 的字幕限制
    cmd.args([
        "--extractor-args",
        "youtube:player_client=android",
        "--write-subs",
        "--write-auto-subs",
        "--skip-download",
        "--sub-langs",
        "en",
        "--sub-format",
        "json3",
        "--no-warnings",
        "-o",
        "%(id)s.%(ext)s",
        &format!("https://www.youtube.com/watch?v={}", video_id),
    ]);

    // 使用临时目录
    let temp_dir = std::env::temp_dir();
    cmd.current_dir(&temp_dir);

    // 执行命令
    let output = cmd.output().map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp failed: {}", stderr));
    }

    // 查找生成的 JSON3 字幕文件
    let json_file = find_subtitle_file(&temp_dir, video_id)?;

    // 读取并解析字幕
    let content = std::fs::read_to_string(&json_file)
        .map_err(|e| format!("Failed to read subtitle file: {}", e))?;

    // 清理临时文件
    let _ = std::fs::remove_file(json_file);

    parse_json3_subtitles(&content, video_id)
}

// 查找字幕文件
fn find_subtitle_file(temp_dir: &std::path::Path, video_id: &str) -> Result<String, String> {
    let entries = std::fs::read_dir(temp_dir)
        .map_err(|e| format!("Failed to read temp dir: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                // 匹配 {video_id}.{lang}.json3 或 {video_id}.json3
                if name.starts_with(video_id) && name.ends_with(".json3") {
                    return path.to_str()
                        .map(|s| s.to_string())
                        .ok_or_else(|| "Invalid path".to_string());
                }
            }
        }
    }

    Err(format!("No subtitle file found for video {}", video_id))
}

// 解析 JSON3 格式字幕
fn parse_json3_subtitles(content: &str, video_id: &str) -> Result<TranscriptResult, String> {
    let json: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let events = json["events"]
        .as_array()
        .ok_or_else(|| "Invalid JSON3 format: missing events".to_string())?;

    let mut items = Vec::new();

    for event in events {
        // JSON3 格式使用 tStartMs 和 dDurationMs（毫秒）
        let start_ms = event["tStartMs"].as_i64();
        let dur_ms = event["dDurationMs"].as_i64();

        if let (Some(start), Some(dur)) = (start_ms, dur_ms) {
            let mut text = String::new();

            if let Some(segs) = event["segs"].as_array() {
                for seg in segs {
                    if let Some(utf8) = seg["utf8"].as_str() {
                        text.push_str(utf8);
                    }
                }
            }

            text = decode_html_entities(&text).trim().to_string();

            if !text.is_empty() {
                items.push(TranscriptItem {
                    text,
                    duration_secs: dur as f64 / 1000.0,
                    offset_secs: start as f64 / 1000.0,
                });
            }
        }
    }

    if items.is_empty() {
        return Err("No valid subtitle segments found".to_string());
    }

    Ok(TranscriptResult {
        video_id: video_id.to_string(),
        items,
    })
}

// 处理 transcript 请求
async fn transcript_handler(
    Json(req): Json<TranscriptRequest>,
) -> Result<Json<TranscriptResult>, ErrorResponse> {
    if req.url.is_empty() {
        return Err(ErrorResponse {
            error: "URL is required".to_string(),
            details: None,
        });
    }

    let video_id = extract_video_id(&req.url)
        .ok_or_else(|| ErrorResponse {
            error: "Invalid YouTube URL".to_string(),
            details: Some("Could not extract video ID".to_string()),
        })?;

    get_transcript_via_ytdlp(&video_id)
        .await
        .map_err(|e| ErrorResponse {
            error: "Failed to fetch transcript".to_string(),
            details: Some(e),
        })
        .map(Json)
}

// 健康检查
async fn health_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    let output = Command::new("yt-dlp")
        .arg("--version")
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let version = String::from_utf8_lossy(&o.stdout);
            Ok(Json(serde_json::json!({
                "status": "healthy",
                "yt-dlp": version.trim()
            })))
        }
        _ => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

// 错误响应实现
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = if self.error.contains("Invalid") {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status, Json(self)).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 检查 yt-dlp 是否安装
    let check = Command::new("yt-dlp").arg("--version").output();
    if check.is_err() || !check.unwrap().status.success() {
        eprintln!("Error: yt-dlp is not installed.");
        eprintln!("Please install it: brew install yt-dlp");
        std::process::exit(1);
    }

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/transcript", post(transcript_handler))
        .route("/health", get(health_handler))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// 优雅关闭处理
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
