use lazy_static::lazy_static;
use std::env;

pub struct TcdtConf {
    pub host: String,
    pub port: String,
    pub security: bool,
    pub enable_change_password: bool,
    pub database_url: String,
    pub tcdt_static: String,
    pub code_template_path: String,
    pub code_generate_path: String,
    pub enable_code_template_edit: bool,
    pub enable_code_generate_debug_mode: bool,
    pub token_duration: i64,
}
lazy_static! {
    pub static ref TCDT_CONF: TcdtConf = {
        dotenvy::from_filename_override("conf/.env").ok();
        let host = env::var("HOST").expect("HOST is not set in .env file");
        let port = env::var("PORT").expect("PORT is not set in .env file");
        let enable_change_password = env::var("ENABLE_CHANGE_PASSWORD").expect("ENABLE_CHANGE_PASSWORD is not set in .env file");
        let security = env::var("SECURITY").expect("SECURITY is not set in .env file");
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let tcdt_static = env::var("TCDT_STATIC").expect("TCDT_STATIC is not set in .env file");
        let code_template_path =
            env::var("CODE_TEMPLATE_PATH").expect("CODE_TEMPLATE_PATH is not set in .env file");
        let code_generate_path =
            env::var("CODE_GENERATE_PATH").expect("CODE_GENERATE_PATH is not set in .env file");
        let enable_code_template_edit =
            env::var("ENABLE_CODE_TEMPLATE_EDIT").expect("ENABLE_CODE_TEMPLATE_EDIT is not set in .env file");
        let enable_code_generate_debug_mode_str =
            env::var("ENABLE_CODE_GENERATE_DEBUG_MODE").expect("ENABLE_CODE_GENERATE_DEBUG_MODE is not set in .env file");
        let token_durtion_str =
            env::var("TOKEN_DURATION").expect("TOKEN_DURATION is not set in .env file");
        let token_durtion = token_durtion_str.parse::<i64>().expect("TOKEN_DURATION need a number type");
        TcdtConf {
            host,
            port,
            security: security.trim() == "true",
            enable_change_password: enable_change_password == "true",
            database_url: db_url,
            code_template_path,
            tcdt_static,
            code_generate_path,
            enable_code_template_edit: enable_code_template_edit.trim() == "true",
            enable_code_generate_debug_mode: enable_code_generate_debug_mode_str.trim() == "true",
            token_duration:token_durtion,
        }
    };
}
