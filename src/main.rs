pub mod util;
pub mod config;

use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Logger, Appender, Config, Root};
use log::LevelFilter;
use log::{info, error};
use actix_web::{HttpServer, App, middleware, web, guard, http};
use actix_http::ResponseBuilder;
use clap::Arg;

//#[macro_use]
//extern crate log4rs;
//extern crate log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let log_stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[Console] {d} - {l} -{t} - {m}{n}",
        )))
        .build();
    let log_config = Config::builder()
        .appender(
            Appender::builder()
                .build("stdout", Box::new(log_stdout),
                )
        )
        .logger(
            Logger::builder()
                .additive(false)
                .build("app", LevelFilter::Info)
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _ = log4rs::init_config(log_config).unwrap();

    let args = clap::App::new("RESTFul Rust Api")
        .version("1.0.0")
        .author("zhang ye bai")
        .arg(Arg::with_name("active")
            .short("a")
            .long("active")
            .takes_value(true)
        )
        .get_matches();
    let active = args.value_of("active").unwrap();
    info!("app active config value is : {}", active);

    let server_config = config::server_config::find_server_config(active);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .guard(guard::Not(guard::Post()))
                    .guard(guard::Not(guard::Put()))
                    .guard(guard::Not(guard::Patch()))
                    .guard(guard::Not(guard::Options()))
                    .guard(guard::Not(guard::Delete()))
                    .to(|| {
                        ResponseBuilder::new(http::StatusCode::OK)
                            .set_header("content-type", "application/json; charset=utf-8")
                            .body("{\"code\":404, \"message\":\"request not found\"}")
                    })
            )
    })

        .bind(format!("{}:{}", server_config.host, server_config.port))?
        .workers(server_config.workers as usize)
        .run()
        .await
}