pub mod region_route;

use actix_web::web;
use crate::route;

pub fn route_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/region")
                .route(
                    "/simple/provinces",
                    web::get().to(route::region_route::list_simple_province),
                )
    /*
                .route(
                    "/cases/source/list",
                    web::get().to(routes::statistics_route::list_case_source_statistics),
                )
                .route(
                    "/cases/sales/list",
                    web::get().to(routes::statistics_route::list_sales_statistics),
                )*/

        );
}