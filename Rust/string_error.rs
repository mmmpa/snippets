//
// convert Result<T, E> to Result<T, String>
//

use std::fmt::Debug;

trait StringError<T> {
    fn str_err(self, str: &str) -> Result<T, String>;
}

impl<T, E: Debug> StringError<T> for Result<T, E> {
    fn str_err(self, str: &str) -> Result<T, String> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("{}: {:?}", str, e))
        }
    }
}
