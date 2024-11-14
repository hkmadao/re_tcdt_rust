// 定义一个结构体来表示错误
#[derive(Debug)]
pub struct TcdtCustomError {
    message: String,
}

impl TcdtCustomError {
    fn new(value: String) -> Self {
        TcdtCustomError { message: value }
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

// 实现 Error trait
impl std::error::Error for TcdtCustomError {}

// 实现 Display trait 以便打印错误信息
impl std::fmt::Display for TcdtCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// 定义一个结构体来表示错误
#[derive(Debug)]
pub struct TcdtInternalError {
    message: String,
}

impl TcdtInternalError {
    fn new(value: String) -> Self {
        TcdtInternalError { message: value }
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

// 实现 Error trait
impl std::error::Error for TcdtInternalError {}

// 实现 Display trait 以便打印错误信息
impl std::fmt::Display for TcdtInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub enum TcdtServiceError {
    TcdtInternal(TcdtInternalError),
    Custom(TcdtCustomError),
}

impl std::error::Error for TcdtServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::TcdtInternal(ref e) => Some(e),
            Self::Custom(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for TcdtServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::TcdtInternal(ref e) => e.fmt(f),
            Self::Custom(ref e) => e.fmt(f),
        }
    }
}

impl From<TcdtInternalError> for TcdtServiceError {
    fn from(value: TcdtInternalError) -> Self {
        Self::TcdtInternal(value)
    }
}

impl From<TcdtCustomError> for TcdtServiceError {
    fn from(value: TcdtCustomError) -> Self {
        Self::Custom(value)
    }
}

impl TcdtServiceError {
    pub fn build_internal_msg(msg: &str) -> Self {
        Self::TcdtInternal(TcdtInternalError::new(format!("{}", msg)))
    }
    pub fn build_internal_msg_error(msg: &str, err: impl std::error::Error) -> Self {
        Self::TcdtInternal(TcdtInternalError::new(format!("{}: {:?}", msg, err)))
    }

    pub fn build_custom_msg(msg: &str) -> Self {
        Self::Custom(TcdtCustomError::new(format!("{}", msg)))
    }

    pub fn build_custom_msg_error(msg: &str, err: impl std::error::Error) -> Self {
        Self::Custom(TcdtCustomError::new(format!("{}: {:?}", msg, err)))
    }

    pub fn build_custom_error(err: impl std::error::Error) -> Self {
        Self::Custom(TcdtCustomError::new(format!("{:?}", err)))
    }
}
