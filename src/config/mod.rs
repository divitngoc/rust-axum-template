use axum::extract::FromRef;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::jwt_config::KEYS;
use crate::domains::applications::repository::ApplicationRepository;
use crate::domains::applications::services::ApplicationService;
use crate::domains::organisations::repository::OrganisationRepository;
use crate::domains::organisations::services::OrganisationService;
use crate::domains::users::repository::UserRepository;
use crate::domains::users::services::UserService;

pub mod diesel_config;
pub mod jwt_config;
pub mod app_env;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub organisation_service: OrganisationService,
    pub application_service: ApplicationService,
}

pub fn init() {
    init_logging();
    app_env::init();
    init_keys();
}

fn init_keys() {
    let _e = &KEYS.encoding;
    let _d = &KEYS.decoding;
}

fn init_logging() -> () {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub(crate) async fn init_app_state() -> AppState {
    let db_pool = diesel_config::establish_connection().await;
    let user_repository = UserRepository::new(db_pool.clone());
    AppState {
        user_service: UserService::new(user_repository.clone()),
        organisation_service: OrganisationService::new(OrganisationRepository::new(db_pool.clone()), user_repository),
        application_service: ApplicationService::new(ApplicationRepository::new(db_pool.clone())),
    }
}