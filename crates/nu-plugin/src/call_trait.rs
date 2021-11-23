use nu_protocol::{ast::Call, FromValue, ShellError};

pub trait CallExtPlugin {
    fn get_flag<T: FromValue>(&self) -> Result<Option<T>, ShellError>;
    fn rest<T: FromValue>(&self) -> Result<Vec<T>, ShellError>;
    fn opt<T: FromValue>(&self, pos: usize) -> Result<Option<T>, ShellError>;
    fn req<T: FromValue>(&self, pos: usize) -> Result<T, ShellError>;
}

impl CallExtPlugin for Call {
    fn get_flag<T: FromValue>(&self) -> Result<Option<T>, ShellError> {
        todo!()
    }

    fn rest<T: FromValue>(&self) -> Result<Vec<T>, ShellError> {
        todo!()
    }

    fn opt<T: FromValue>(&self, pos: usize) -> Result<Option<T>, ShellError> {
        todo!()
    }

    fn req<T: FromValue>(&self, pos: usize) -> Result<T, ShellError> {
        todo!()
    }
}
