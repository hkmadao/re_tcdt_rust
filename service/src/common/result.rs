use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppResult<T> {
    status: i32,
    message: String,
    data: Option<T>,
}

impl<T> AppResult<T> {
    pub fn new(code: i32, message: String, data: T) -> AppResult<T> {
        AppResult {
            status: code,
            message,
            data: Some(data),
        }
    }

    pub fn success(data: T) -> AppResult<T> {
        AppResult {
            status: 0,
            message: String::from(""),
            data: Some(data),
        }
    }

    pub fn success_not_data() -> AppResult<T> {
        AppResult {
            status: 0,
            message: String::from(""),
            data: None,
        }
    }

    pub fn success_msg(message: String) -> AppResult<T> {
        AppResult {
            status: 0,
            message,
            data: None,
        }
    }

    pub fn success_msg_and_data(message: String, data: T) -> AppResult<T> {
        AppResult {
            status: 0,
            message,
            data: Some(data),
        }
    }

    pub fn failed_msg(message: String) -> AppResult<T> {
        AppResult {
            status: 1,
            message,
            data: None,
        }
    }

    pub fn failed_msg_and_data(message: String, data: T) -> AppResult<T> {
        AppResult {
            status: 1,
            message,
            data: Some(data),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfoInput {
    page_index: u64,
    page_size: u64,
    total_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo<T> {
    page_info_input: PageInfoInput,
    data_list: Vec<T>,
}

impl<T> PageInfo<T> {
    pub fn new(page_index: u64, page_size: u64, total: u64, data: Vec<T>) -> PageInfo<T> {
        PageInfo {
            page_info_input: PageInfoInput {
                page_index,
                page_size,
                total_count: total,
            },
            data_list: data,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRefErrorMessageVO {
    /// 被引用id
    pub id_data: String,
    /// 错误信息
    pub message: String,
    /// 被引用类名
    pub source_class_name: String,
    /// 引用类名
    pub ref_class_name: String,
}

pub enum SaveResult<T> {
    Ok(T),
    ErrMsg(Vec<DeleteRefErrorMessageVO>),
    None(),
}
