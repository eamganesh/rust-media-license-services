mod authentication;

use axum::{
    extract::{self, Path, State},
    middleware,
    routing::get,
    Extension, Router,
};
use cloudfront_sign::SignedOptions;
use jsonwebtoken::DecodingKey;
use std::{net::SocketAddr, sync::Arc};

use crate::signer::*;

use self::authentication::JWT_PASS_CODE;
#[tokio::main]
pub async fn start_server() {
    // Initializing App state with sign options
    // Important to initialize here to reduce heap allocations
    let sign_options: SignedOptions = SignedOptions {
        key_pair_id: String::from(CLOUDFRONT_KEY_PAIR_ID),
        private_key: String::from(CLOUDFRONT_PRIVATE_KEY),
        ..Default::default()
    };
    let app_state: Arc<AppState> = Arc::new(AppState {
        sign_options,
        jwt_decoding_key: DecodingKey::from_secret(JWT_PASS_CODE.as_bytes()),
    });
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route(
            "/v1/sign/:name",
            get(root).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                crate::server::authentication::authenticate,
            )),
        )
        .with_state(app_state);
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn root(
    Extension(jwt_claim): Extension<authentication::JwtClaim>,
    Path(name): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> String {
    let url_to_sign: String = format!("{}/{}", &jwt_claim.data.baseUrl, &name);
    crate::signer::sign(&url_to_sign, &app_state)
}

pub struct AppState {
    pub sign_options: SignedOptions,
    pub jwt_decoding_key: DecodingKey,
}
