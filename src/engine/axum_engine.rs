use std::sync::Arc;

use crate::{
    config::{
        self,
        db::{Connection, Sources},
    },
    environment::Environment,
    errors::Result,
    router::axum_router::{auth, gym, gymnast, midleware::jwt_auth::auth},
    services::{auth::AuthServices, gym::GymServices, gymnast::GymnastServices},
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use redis::Client;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub gym_services: GymServices,
    pub gymnast_services: GymnastServices,
    pub auth_services: AuthServices,
    pub redis_client: Client,
    pub environment: Environment,
}

pub async fn run() -> Result<()> {
    let mut surreal_db = config::db::DatabaseSource {
        db_type: config::db::DatabaseType::SurrealDB,
    };

    let redis_client = match Client::open("redis://localhost") {
        Ok(client) => {
            println!("✅ Connection to Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to the database
    let conn = Arc::new(surreal_db.connect().await?);
    let ping_db = conn.ping();

    if ping_db == String::from("Pong!") {
        println!("✅ {} from database!", ping_db);
    } else {
        println!("🔥 {} from database!", ping_db);
        std::process::exit(1);
    }

    let gym_services = GymServices { repo: conn.clone() };
    let gymnast_services = GymnastServices { repo: conn.clone() };
    let auth_services = AuthServices { repo: conn.clone() };
    let environment = Environment::new();

    let app_state = AppState {
        gym_services,
        gymnast_services,
        auth_services,
        redis_client,
        environment,
    };

    let shared_state = Arc::new(app_state);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let routes_all = Router::new()
        .merge(gym_routes(shared_state.clone()))
        .merge(auth_routes(shared_state.clone()))
        .merge(gymnast_routes(shared_state))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}


pub fn gym_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/gym", post(gym::register))
        .route(
            "/api/v1/gym/:id",
            get(gym::get_profile)
                .put(gym::update_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/login", post(auth::login_user))
        .with_state(app_state)
}

pub fn gymnast_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/gymnast", post(gymnast::register))
        .with_state(app_state)
}
