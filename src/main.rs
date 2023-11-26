use std::{fmt::{Debug, Display}, error::Error};


enum CustomError {
    FileReadError(std::io::Error),
    RequestError(reqwest::Error),
    FileDeleteError(std::io::Error)
}


impl From<reqwest::Error> for CustomError {
    fn from(e: reqwest::Error) -> Self {
       CustomError::RequestError(e) 
    }
}

impl Error for CustomError {

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use CustomError::*;
        match self {
            FileReadError(s) => Some(s),
            RequestError(s) => Some(s),
            FileDeleteError(s) => Some(s)
        }
    }
}


impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "This is caused by: {}", source)?;
        }

        Ok(())
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomError::*;
        match self {
            FileReadError(_) => writeln!(f, "failed to read the key file"),
            RequestError(_) => writeln!(f, "failed to request on the url"),
            FileDeleteError(_) => writeln!(f, "failed to delete the key file"),
        }
    }
}

fn request() -> Result<(), CustomError> {
    use CustomError::*;

    let key = std::fs::read_to_string("sample-key").map_err(FileReadError)?;
    reqwest::blocking::get(format!("https://httpbin.dev/key/{}", "sample"))
        .map_err(RequestError)?
        .error_for_status()
        .map_err(RequestError)?;

    std::fs::remove_file("sample-file").map_err(FileDeleteError)?;
    Ok(())
}

fn main() {
    println!("{:?}", request().unwrap_err());
}

