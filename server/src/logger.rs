use homedir::get_my_home;
use tracing_appender::non_blocking::WorkerGuard;

const LOG_FILE_NAME: &str = "markdown-lsp-logs.log";
const LOG_FILE_DIR_NAME: &str = "markdown-lsp-logs";

// XXX I believe logs go to neovim if they are printed on stderr
pub fn init_logger() -> WorkerGuard {
    let home = get_my_home().expect("could not get HOME dir").expect("no HOME dir set");
    let mut log_file_dir = home;
    log_file_dir.push(LOG_FILE_DIR_NAME);
    let file_appender = tracing_appender::rolling::never(log_file_dir, LOG_FILE_NAME);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    _guard
}
