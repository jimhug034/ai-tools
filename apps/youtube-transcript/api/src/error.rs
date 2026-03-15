//! 错误类型定义

use thiserror::Error;

pub type Result<T> = std::result::Result<T, YtError>;

#[derive(Error, Debug)]
pub enum YtError {
    #[error("无效的 YouTube URL: {0}")]
    InvalidUrl(String),

    #[error("无法提取视频 ID")]
    VideoIdExtractionFailed,

    #[error("无法获取视频页面: {0}")]
    FetchFailed(String),

    #[error("无法解析页面数据: {0}")]
    ParseError(String),

    #[error("未找到字幕")]
    NoCaptionsFound,

    #[error("字幕下载失败: {0}")]
    DownloadFailed(String),

    #[error("格式转换失败: {0}")]
    ConversionError(String),

    #[error("HTTP 请求错误: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}
