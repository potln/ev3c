#[derive(Debug)]
pub enum ErrorKind {
    ArgumentError,
    FileError,
    LexError,
    ParseError,
}

#[derive(Debug)]
pub struct Error {
    message: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub fn new(message: &'static str, kind: ErrorKind) -> Error {
        return Error {
            message: message,
            kind: kind,
        };
    }

    pub fn format(self) -> String {
        return format!("{:?}: {}", self.kind, self.message);
    }
}
