use lazy_static::lazy_static;
use std::env;

pub struct TcdtConf {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub tcdt_static: String,
    pub code_template_path: String,
    pub code_generate_path: String,
    pub enable_code_generate_debug_mode: bool,
}
lazy_static! {
    pub static ref TCDT_CONF: TcdtConf = {
        dotenvy::from_filename_override("conf/.env").ok();
        let host = env::var("HOST").expect("HOST is not set in .env file");
        let port = env::var("PORT").expect("PORT is not set in .env file");
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let tcdt_static = env::var("TCDT_STATIC").expect("TCDT_STATIC is not set in .env file");
        let code_template_path =
            env::var("CODE_TEMPLATE_PATH").expect("CODE_TEMPLATE_PATH is not set in .env file");
        let code_generate_path =
            env::var("CODE_GENERATE_PATH").expect("CODE_GENERATE_PATH is not set in .env file");
        let enable_code_generate_debug_mode_str =
            env::var("ENABLE_CODE_GENERATE_DEBUG_MODE").expect("ENABLE_CODE_GENERATE_DEBUG_MODE is not set in .env file");
        TcdtConf {
            host: host,
            port: port,
            database_url: db_url,
            code_template_path: code_template_path,
            tcdt_static: tcdt_static,
            code_generate_path: code_generate_path,
            enable_code_generate_debug_mode: enable_code_generate_debug_mode_str.trim() == "true",
        }
    };
}
