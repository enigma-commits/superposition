mod api;
mod db;
mod utils;
mod v1;

use api::primary::{
    context_overrides::{delete_ctx_override, get_ctx_override, post_ctx_override},
    dimensions::{get_dimension_key, get_dimensions, post_dimension},
    global_config::{get_global_config, get_global_config_key, post_config_key_value},
    overrides::{delete_override, get_override, post_override},
};

use api::derived::{
    config::get_config, context_override::add_new_context_override,
    promote::promote_contexts_overrides, reduce::reduce_contexts_overrides,
};

use dotenv;
use std::env;
use std::io::Result;

use actix::SyncArbiter;
use actix_web::{middleware::Logger, web::scope, web::Data, App, HttpServer};
use db::utils::{get_pool, AppState, DbActor};

use v1::api::*;

#[actix_web::main]
async fn main() -> Result<()> {
    // just_for_test();
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");
    let pool = get_pool(&db_url);
    let pool_cl = pool.clone();
    let db_addr = SyncArbiter::start(5, move || DbActor(pool_cl.clone()));
    HttpServer::new(move || {
        let logger: Logger = Logger::default();
        App::new()
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                db_pool: pool.clone(),
            }))
            .wrap(logger)
            /***************************** Primary api routes *****************************/
            .service(
                scope("/global_config")
                    .service(get_global_config)
                    .service(get_global_config_key)
                    .service(post_config_key_value),
            )
            .service(
                scope("/dimensions")
                    .service(get_dimensions)
                    .service(get_dimension_key)
                    .service(post_dimension),
            )
            .service(
                scope("/context_overrides")
                    .service(post_ctx_override)
                    .service(delete_ctx_override)
                    .service(get_ctx_override),
            )
            .service(
                scope("/override")
                    .service(post_override)
                    .service(delete_override)
                    .service(get_override),
            )
            .service(scope("/context").service(context::endpoints()))
            /***************************** Derived api routes *****************************/
            .service(scope("/config").service(get_config))
            .service(scope("add_context_overrides").service(add_new_context_override))
            .service(scope("reduce").service(reduce_contexts_overrides))
            .service(scope("promote").service(promote_contexts_overrides))
    })
    .bind(("localhost", 8080))?
    .workers(5)
    .run()
    .await
}
