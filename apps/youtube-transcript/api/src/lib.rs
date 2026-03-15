//! YouTube 字幕处理库
//!
//! 参考 yt-dlp 实现的 Rust 版 YouTube 字幕下载和转换功能

pub mod error;
pub mod types;
pub mod extractor;
pub mod downloader;
pub mod converter;

pub use error::{Result, YtError};
pub use types::*;
