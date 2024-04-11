use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

/// 本地时间格式化
pub struct LocalTimer;

impl LocalTimer {
    /// 返回东八区（中国标准时间）的固定偏移量
    fn east8() -> FixedOffset {
        FixedOffset::east_opt(8 * 3600).expect("东八区时区偏移量应该是有效的")
    }

    // 添加一个新方法来转换任意UTC时间为东八区时间
    #[allow(dead_code)]
    pub fn convert_to_east8(dt: DateTime<Utc>) -> DateTime<FixedOffset> {
        dt.with_timezone(&Self::east8())
    }
}

impl FormatTime for LocalTimer {
    /// 格式化时间为东八区时间，并写入给定的 Writer
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&Self::east8());
        write!(w, "{}", now.format("%FT%T%.3f"))
    }
}
