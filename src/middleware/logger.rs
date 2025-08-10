use chrono_tz::Asia::Shanghai;
use owo_colors::OwoColorize;
use std::{env, fmt};
use tracing::Subscriber;
use tracing_subscriber::{
    Layer,
    filter::LevelFilter,
    fmt::{FormatEvent, FormatFields},
    layer::SubscriberExt,
    registry::LookupSpan,
};
struct Formatter {
    show_target: bool,
}

impl<S, N> FormatEvent<S, N> for Formatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let prefix = "[PIC-IMAGE-API]".magenta().to_string();
        write!(writer, "{} ", prefix)?;

        let local_time = chrono::Local::now();
        let shanghai_time = local_time.with_timezone(&Shanghai);
        let formatted_time = shanghai_time.format("%H:%M:%S%.3f");
        write!(writer, "[{}] ", formatted_time)?;

        let logger_level = event.metadata().level();
        let colored_level = match *logger_level {
            tracing::Level::ERROR => logger_level.red().to_string(),
            tracing::Level::WARN => logger_level.yellow().to_string(),
            tracing::Level::INFO => logger_level.green().to_string(),
            tracing::Level::DEBUG => logger_level.blue().to_string(),
            tracing::Level::TRACE => logger_level.magenta().to_string(),
        };
        write!(writer, "[{: <17}] ", colored_level)?;

        if self.show_target {
            write!(writer, "[{}] ", event.metadata().target().purple())?;
        }

        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

pub fn log_init() {
    let debug_mode = env::var("DEBUG")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);
    let level = if debug_mode {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let logger_level = match level {
        LevelFilter::OFF => LevelFilter::OFF,
        LevelFilter::ERROR => LevelFilter::ERROR,
        LevelFilter::WARN => LevelFilter::WARN,
        LevelFilter::INFO => LevelFilter::INFO,
        LevelFilter::DEBUG => LevelFilter::DEBUG,
        LevelFilter::TRACE => LevelFilter::TRACE,
    };

    let show_target = matches!(logger_level, LevelFilter::DEBUG | LevelFilter::TRACE);

    // 创建控制台日志层
    let console_subscriber = tracing_subscriber::fmt::layer()
        .event_format(Formatter { show_target })
        .with_filter(logger_level);

    let subscriber = tracing_subscriber::registry().with(console_subscriber);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing_log::LogTracer::init().unwrap();
}
