pub(super) struct Cors(tower_http::cors::CorsLayer);

impl Cors {
    pub fn new() -> Self {
        let cors = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
            .allow_methods(tower_http::cors::Any);

        Self(cors)
    }

    pub fn get_layer(self) -> tower_http::cors::CorsLayer {
        self.0
    }
}

impl std::ops::Deref for Cors {
    type Target = tower_http::cors::CorsLayer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Cors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
