use thiserror::Error;

#[derive(Error, Debug)]
pub enum GffError {
    #[error("stdin not detected")]
    StdinNotDetected,
    #[error("faidx not exists {0}")]
    FaidxNotExists(#[from] std::io::Error),
}
