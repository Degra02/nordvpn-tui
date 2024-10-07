
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Command(std::process::ExitStatus),
    Utf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}
