use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::post_service,
    Router,
};
use config::db_config::Config;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::schema::resolver::{Mutation, QueryRoot};

pub mod config;
pub mod schema;

pub struct DbState {
    pub db: Pool<Postgres>,
    pub env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let schema = Schema::build(QueryRoot::default(), Mutation::default(), EmptySubscription)
        .data(DbState {
            db: pool.clone(),
            env: config.clone(),
        })
        .finish();
    let app = Router::new()
        .route("/graphql", post_service(GraphQL::new(schema)))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]),
        );
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
