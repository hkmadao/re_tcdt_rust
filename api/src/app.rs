use std::rc::Rc;
use std::sync::Arc;
use actix_files::Files as Fs;
use actix_http::body::MessageBody;
use actix_web::{middleware::{self}, web, App, Error, Handler, HttpRequest, HttpResponse, HttpServer, Result};
use listenfd::ListenFd;
use tcdt_common::tcdt_conf::TCDT_CONF;
use tcdt_service::sea_orm::{Database, DatabaseConnection, EntityTrait};

use crate::{api_register::go_register, conf::{response_handle::ResponseHandler, cors_handle::CorsHandler, security_handle::SecurityHandler}};

#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

// fn setup_logger() -> Result<(), fern::InitError> {
//     fern::Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "{}[{}][{}] {}",
//                 chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//                 record.target(),
//                 record.level(),
//                 message
//             ))
//         })
//         .level(log::LevelFilter::Debug)
//         .chain(std::io::stdout())
//         .chain(fern::log_file("output.log")?)
//         .apply()?;
//     Ok(())
// }

// fn setup_tracing_subscriber() {
//     // 设置日志文件路径和日志滚动策略
//     let rolling_file = tracing_appender::rolling::RollingFileAppender::new(
//         tracing_appender::rolling::Rotation::DAILY,
//         "./logs/",
//         "tcdt.log",
//     );
//     let (non_blocking, _guard) = tracing_appender::non_blocking(rolling_file);

//     tracing_subscriber::fmt()
//         .with_writer(non_blocking)
//         .with_level(true)
//         .with_line_number(true)
//         .with_ansi(true)
//         .finish()
//         .init();
// }

fn setup_log4rs() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
}

async fn not_found(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let method = request.method().clone();
    if method.to_string() == "OPTIONS" {
        return Ok(HttpResponse::Ok().body(""));
    }
    let mut ctx = tera::Context::new();
    ctx.insert("uri", request.uri().path());

    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body("404 Not Found"))
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    // log record
    // setup_tracing_subscriber();
    setup_log4rs();
    // setup_logger().ok();

    // get env vars
    let db_url = &TCDT_CONF.database_url;
    let host = &TCDT_CONF.host;
    let port = &TCDT_CONF.port;
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url).await.unwrap();

    let state = AppState { conn };

    let web_data = web::Data::new(state);

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .service(Fs::new("/tcdt", &TCDT_CONF.tcdt_static))
            // .service(Fs::new("/tcdt", "./api/static/dist"))
            .app_data(web_data.clone())
            .wrap(CorsHandler {})
            .wrap(SecurityHandler {})
            .wrap(ResponseHandler {})
            .wrap(middleware::Logger::default()) // enable logger
            .default_service(web::route().to(not_found))
            .configure(go_register)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

