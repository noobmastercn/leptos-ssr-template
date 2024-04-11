use std::path::Path;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::fmt::format::FmtSpan;
use crate::util::local_timer::LocalTimer;

// tracing使用教程 https://rust-book.junmajinlong.com/ch102/tracing.html
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_timer(LocalTimer)
        .with_max_level(Level::INFO)
        .init();
}

pub fn init_logging_file(
    directory: impl AsRef<Path>,
    file_name_prefix: impl AsRef<Path>,
) -> WorkerGuard {
    let file_appender = rolling::daily(directory, file_name_prefix);
    let (non_blocking, guard) = non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        // FmtSpan::NONE：不记录任何 span 生命周期事件。
        // FmtSpan::NEW：仅在 span 创建时记录。
        // FmtSpan::CLOSE：仅在 span 关闭时记录。
        // FmtSpan::ACTIVE：在 span 激活时记录。
        // FmtSpan::FULL：记录 span 的所有生命周期事件（创建、激活、关闭）。
        .with_span_events(FmtSpan::FULL)
        .init();
    guard
}
