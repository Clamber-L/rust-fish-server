use crate::utils::result::HttpResult;

pub mod result;

pub type Res = HttpResult<String>;
pub type ResData<T> = HttpResult<T>;
