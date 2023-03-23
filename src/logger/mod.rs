pub(crate) mod log_level;
pub(crate) mod log;
pub(crate) mod macros;

pub(crate) struct Builder {
    level: log_level::LogLevel,
    write_to_console: bool,
    write_to_file: bool,
    file_path: String,
}

#[allow(dead_code)]
impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            level: log_level::LogLevel::Info,
            write_to_console: true,
            write_to_file: false,
            file_path: "".to_string(),
        }
    }

    pub(crate) fn level(&mut self, level: log_level::LogLevel) -> &Builder {
        self.level = level;
        self
    }

    pub(crate) fn write_to_console(&mut self, write_to_console: bool) -> &Builder {
        self.write_to_console = write_to_console;
        self
    }

    pub(crate) fn write_to_file(&mut self, write_to_file: bool) -> &Builder {
        self.write_to_file = write_to_file;
        self
    }

    pub(crate) fn file_path(&mut self, file_path: &str) -> &Builder {
        self.file_path = file_path.to_string();
        self
    }

    pub(crate) fn build(&self) -> &Builder {
        init(self);
        self
    }
}

pub(crate) fn init(options: &Builder) {
    // set the options as environment variables
    std::env::set_var("ALVEUS_LOG", options.level.to_string());
    std::env::set_var("ALVEUS_LOG_CONSOLE", options.write_to_console.to_string());
    std::env::set_var("ALVEUS_LOG_FILE", options.write_to_file.to_string());
    std::env::set_var("ALVEUS_LOG_FILE_PATH", options.file_path.to_string());
}