use thiserror::Error;
#[derive(Error, Debug)]
pub enum KyoError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),
}

// ill need to add errors for like.. everything in the C# version but this is a start LOL
