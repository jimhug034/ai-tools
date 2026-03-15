use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

// 导入库模块
use youtube_transcript::{
    converter::SubtitleConverter,
    downloader::CaptionDownloader,
    error::YtError,
    extractor::{extract_video_id, CaptionExtractor},
    types::{CaptionKind, SubtitleFormat},
};

// HTTP 客户端（单例）
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Accept-Language", reqwest::header::HeaderValue::from_static("en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7"));
            headers.insert("Accept", reqwest::header::HeaderValue::from_static("*/*"));
            headers.insert("Sec-Fetch-Site", reqwest::header::HeaderValue::from_static("same-origin"));
            headers.insert("Sec-Fetch-Mode", reqwest::header::HeaderValue::from_static("cors"));
            headers.insert("Sec-Fetch-Dest", reqwest::header::HeaderValue::from_static("empty"));
            headers
        });

    // 配置代理
    if let Ok(proxy) = std::env::var("HTTPS_PROXY") {
        if let Ok(proxy_url) = Proxy::all(&proxy) {
            builder = builder.proxy(proxy_url);
            eprintln!("Proxy configured: {}", proxy);
        }
    } else if let Ok(proxy) = std::env::var("HTTP_PROXY") {
        if let Ok(proxy_url) = Proxy::all(&proxy) {
            builder = builder.proxy(proxy_url);
            eprintln!("Proxy configured: {}", proxy);
        }
    }

    builder.build().unwrap()
});

// 应用状态
#[derive(Clone)]
struct AppState {
    extractor: Arc<CaptionExtractor>,
    downloader: Arc<CaptionDownloader>,
}

// ==================== API 数据结构 ====================

#[derive(Debug, Deserialize)]
struct TranscriptRequest {
    url: String,
}

#[derive(Debug, Deserialize)]
struct FormatRequest {
    url: String,
    format: String,
}

#[derive(Debug, Deserialize)]
struct InfoRequest {
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
struct SubtitleFormatResult {
    format: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct CaptionTrackInfo {
    base_url: String,
    language_code: String,
    name: Option<String>,
    kind: String,
}

#[derive(Debug, Serialize)]
struct VideoInfoResult {
    video_id: String,
    title: Option<String>,
    url: String,
    available_captions: Vec<CaptionTrackInfo>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

// ==================== 辅助函数 ====================

fn map_yt_error(e: YtError) -> ErrorResponse {
    ErrorResponse {
        error: match &e {
            YtError::InvalidUrl(_) => "Invalid YouTube URL".to_string(),
            YtError::VideoIdExtractionFailed => "Invalid YouTube URL".to_string(),
            YtError::NoCaptionsFound => "No captions available".to_string(),
            YtError::FetchFailed(_) => "Failed to fetch video".to_string(),
            YtError::DownloadFailed(_) => "Failed to download captions".to_string(),
            YtError::ParseError(_) => "Failed to parse data".to_string(),
            _ => "Internal error".to_string(),
        },
        details: Some(e.to_string()),
    }
}

// ==================== HTTP 处理器 ====================

async fn transcript_handler(
    State(state): State<AppState>,
    Json(req): Json<TranscriptRequest>,
) -> Result<Json<TranscriptResult>, ErrorResponse> {
    if req.url.is_empty() {
        return Err(ErrorResponse {
            error: "URL is required".to_string(),
            details: None,
        });
    }

    let video_id = extract_video_id(&req.url)
        .map_err(|_| ErrorResponse {
            error: "Invalid YouTube URL".to_string(),
            details: Some("Could not extract video ID".to_string()),
        })?;

    // 获取字幕轨道并立即下载（避免签名过期）
    // 最多重试 3 次，因为字幕 URL 签名可能过期
    let mut subtitle_data = None;
    let mut last_error = None;

    for attempt in 1..=3 {
        let tracks = state.extractor
            .extract_caption_tracks(&video_id)
            .await
            .map_err(map_yt_error)?;

        if tracks.is_empty() {
            return Err(ErrorResponse {
                error: "No captions found".to_string(),
                details: Some("This video has no available captions".to_string()),
            });
        }

        // 优先选择非自动字幕，然后是英文，最后是第一个可用字幕
        let track = tracks
            .iter()
            .filter(|t| t.kind == CaptionKind::Manual)
            .find(|t| t.language_code == "en" || t.language_code.starts_with("en-"))
            .or_else(|| tracks.iter().find(|t| t.kind == CaptionKind::Manual))
            .or_else(|| tracks.iter().find(|t| t.language_code == "en" || t.language_code.starts_with("en-")))
            .or_else(|| tracks.first())
            .ok_or_else(|| ErrorResponse {
                error: "No captions found".to_string(),
                details: None,
            })?;

        eprintln!(
            "Attempt {}: Using caption track: {} ({})",
            attempt,
            track.name.as_ref().unwrap_or(&track.language_code),
            track.language_code
        );

        // 下载字幕
        match state.downloader.download(track).await {
            Ok(data) => {
                subtitle_data = Some(data);
                break;
            }
            Err(e) => {
                let error_msg = e.to_string();
                eprintln!("Download attempt {} failed: {}", attempt, error_msg);
                last_error = Some(e);
                // 如果不是 URL 过期错误，直接返回
                if !error_msg.contains("过期") && !error_msg.contains("内容为空") {
                    break;
                }
            }
        }
    }

    let subtitle_data = subtitle_data.ok_or_else(|| {
        last_error.map(map_yt_error).unwrap_or_else(|| ErrorResponse {
            error: "Failed to fetch transcript".to_string(),
            details: Some("All download attempts failed".to_string()),
        })
    })?;

    // 转换为 API 格式
    let items = subtitle_data
        .entries
        .into_iter()
        .map(|entry| {
            let duration_secs = entry.duration_secs();
            let offset_secs = entry.start_secs();
            TranscriptItem {
                text: entry.text,
                duration_secs,
                offset_secs,
            }
        })
        .collect();

    Ok(Json(TranscriptResult {
        video_id,
        items,
    }))
}

async fn format_handler(
    State(state): State<AppState>,
    Query(req): Query<FormatRequest>,
) -> Result<Json<SubtitleFormatResult>, ErrorResponse> {
    if req.url.is_empty() {
        return Err(ErrorResponse {
            error: "URL is required".to_string(),
            details: None,
        });
    }

    let format = SubtitleFormat::parse(&req.format).ok_or_else(|| ErrorResponse {
        error: "Invalid format".to_string(),
        details: Some(format!(
            "Unsupported format: {}. Supported: srt, vtt, ass, txt, lrc",
            req.format
        )),
    })?;

    let video_id = extract_video_id(&req.url).map_err(|_| ErrorResponse {
        error: "Invalid YouTube URL".to_string(),
        details: Some("Could not extract video ID".to_string()),
    })?;

    // 获取字幕轨道
    let tracks = state
        .extractor
        .extract_caption_tracks(&video_id)
        .await
        .map_err(map_yt_error)?;

    let track = tracks
        .iter()
        .filter(|t| t.kind == CaptionKind::Manual)
        .find(|t| t.language_code == "en" || t.language_code.starts_with("en-"))
        .or_else(|| tracks.iter().find(|t| t.kind == CaptionKind::Manual))
        .or_else(|| tracks.iter().find(|t| t.language_code == "en" || t.language_code.starts_with("en-")))
        .or_else(|| tracks.first())
        .ok_or_else(|| ErrorResponse {
            error: "No captions found".to_string(),
            details: None,
        })?;

    // 下载并转换字幕
    let subtitle_data = state
        .downloader
        .download(track)
        .await
        .map_err(map_yt_error)?;

    let content = SubtitleConverter::convert(&subtitle_data, format).map_err(|e| ErrorResponse {
        error: "Format conversion failed".to_string(),
        details: Some(e.to_string()),
    })?;

    Ok(Json(SubtitleFormatResult {
        format: req.format,
        content,
    }))
}

async fn info_handler(
    State(state): State<AppState>,
    Query(req): Query<InfoRequest>,
) -> Result<Json<VideoInfoResult>, ErrorResponse> {
    if req.url.is_empty() {
        return Err(ErrorResponse {
            error: "URL is required".to_string(),
            details: None,
        });
    }

    let video_id = extract_video_id(&req.url).map_err(|_| ErrorResponse {
        error: "Invalid YouTube URL".to_string(),
        details: Some("Could not extract video ID".to_string()),
    })?;

    // 获取视频信息
    let video_info = state
        .extractor
        .extract_video_info(&req.url)
        .await
        .map_err(map_yt_error)?;

    // 获取可用字幕
    let tracks = state
        .extractor
        .extract_caption_tracks(&video_id)
        .await
        .map_err(map_yt_error)?;

    let available_captions = tracks
        .into_iter()
        .map(|t| CaptionTrackInfo {
            base_url: t.base_url,
            language_code: t.language_code,
            name: t.name,
            kind: if t.kind == CaptionKind::Auto {
                "auto".to_string()
            } else {
                "manual".to_string()
            },
        })
        .collect();

    Ok(Json(VideoInfoResult {
        video_id,
        title: video_info.title,
        url: video_info.url,
        available_captions,
    }))
}

async fn formats_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "formats": [
            {"name": "SRT", "value": "srt", "description": "SubRip subtitle format"},
            {"name": "VTT", "value": "vtt", "description": "WebVTT subtitle format"},
            {"name": "ASS", "value": "ass", "description": "Advanced SubStation Alpha format"},
            {"name": "TXT", "value": "txt", "description": "Plain text format"},
            {"name": "LRC", "value": "lrc", "description": "Lyrics format"}
        ]
    }))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "implementation": "pure-rust",
        "features": [
            "caption_extraction",
            "caption_download",
            "format_conversion"
        ]
    }))
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.error.as_str() {
            "Invalid YouTube URL" | "Invalid format" | "URL is required" => {
                StatusCode::BAD_REQUEST
            }
            "No captions found" | "No captions available" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // 创建服务实例
    let state = AppState {
        extractor: Arc::new(CaptionExtractor::with_client(HTTP_CLIENT.clone())),
        downloader: Arc::new(CaptionDownloader::with_client(HTTP_CLIENT.clone())),
    };

    let app = Router::new()
        .route("/transcript", post(transcript_handler))
        .route("/format", get(format_handler))
        .route("/info", get(info_handler))
        .route("/formats", get(formats_handler))
        .route("/health", get(health_handler))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
