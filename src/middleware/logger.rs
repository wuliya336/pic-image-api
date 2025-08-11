use chrono_tz::Asia::Shanghai;
use owo_colors::OwoColorize;
use std::{env, fmt};
use tracing::Subscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    Layer,
    filter::LevelFilter,
    fmt::{FormatEvent, FormatFields},
    layer::SubscriberExt,
    registry::LookupSpan,
};
struct Formatter {
    color: bool,
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

        let prefix = if self.color {
            "[PIC-IMAGE-API]".magenta().to_string()
        } else {
            "[PIC-IMAGE-API]".to_string()
        };
        write!(writer, "{} ", prefix)?;

        let local_time = chrono::Local::now();
        let shanghai_time = local_time.with_timezone(&Shanghai);
        let formatted_time = shanghai_time.format("%H:%M:%S%.3f");
        write!(writer, "[{}] ", formatted_time)?;

        let logger_level = event.metadata().level();
        if self.color {
            let colored_level = match *logger_level {
                tracing::Level::ERROR => logger_level.red().to_string(),
                tracing::Level::WARN => logger_level.yellow().to_string(),
                tracing::Level::INFO => logger_level.green().to_string(),
                tracing::Level::DEBUG => logger_level.blue().to_string(),
                tracing::Level::TRACE => logger_level.magenta().to_string(),
            };
            write!(writer, "[{: <17}] ", colored_level)?;
        } else {
            write!(writer, "[{: <7}] ", logger_level)?;
        }


        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

pub fn log_init() {
    let debug_mode = env::var("DEBUG")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    let logger_level = if debug_mode {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let console_subscriber = tracing_subscriber::fmt::layer()
        .event_format(Formatter { color: true })
        .with_filter(logger_level);

    let mut layers = vec![console_subscriber.boxed()];

    let log_dir = "logs".to_string();
    let _ = std::fs::create_dir_all(&log_dir);
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("pic-image-api")
        .filename_suffix("log")
        .max_log_files(7)
        .build(&log_dir)
        .unwrap();

    let file_subscriber = tracing_subscriber::fmt::layer()
        .event_format(Formatter { color: false })
        .with_writer(file_appender)
        .with_ansi(false)
        .with_filter(logger_level);

    layers.push(file_subscriber.boxed());

    let subscriber = tracing_subscriber::registry().with(layers);

    if tracing::subscriber::set_global_default(subscriber).is_err() || tracing_log::LogTracer::init().is_err(){
        return;
    }
}
