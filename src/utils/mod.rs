use crate::utils::result::HttpResult;

pub mod result;

pub type ResMessage = HttpResult<String>;
pub type ResData<T> = HttpResult<T>;
