pub mod region_route;
pub mod file_route;

use actix_web::web;
use crate::route;

pub fn route_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/region")
                .route(
                    "/simple/provinces",
                    web::get().to(route::region_route::list_simple_provinces),
                )
                .route(
                    "/simple/cities",
                    web::get().to(route::region_route::list_simple_cities_by_province_id)
                )
                .route(
                    "/simple/counties",
                    web::get().to(route::region_route::list_simple_counties_by_city_id)
                )
                .route(
                    "/simple/towns",
                    web::get().to(route::region_route::list_simple_towns_by_county_id)
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

        )

        .service(
            web::scope("/api/file")
                .route(
                    "/simple/file",
                    web::get().to(file_route::find_file_by_file_id)
                )
                .route(
                    "/simple/file",
                    web::post().to(file_route::file_upload)
                )
        );
}