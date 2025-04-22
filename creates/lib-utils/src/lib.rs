use crate::result::HttpResult;

pub mod password;
pub mod result;

pub type ResMessage = HttpResult<String>;
pub type ResData<T> = HttpResult<T>;
