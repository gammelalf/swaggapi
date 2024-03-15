use std::sync::Arc;

use swagger_ui::UrlObject;

use crate::internals::{AccessSwaggapiPageBuilder, SwaggapiPageBuilder};
use crate::{PageOfEverything, SwaggapiPage};

pub struct SwaggerUi {
    path: &'static str,
    config: swagger_ui::Config,
    pages: Vec<(&'static str, &'static str, &'static SwaggapiPageBuilder)>,
}
impl Default for SwaggerUi {
    fn default() -> Self {
        Self {
            path: "/swagger-ui",
            config: swagger_ui::Config::default(),
            pages: vec![("Entire API", "openapi.json", PageOfEverything::builder())],
        }
    }
}
impl SwaggerUi {
    /// Adds a [`SwaggapiPage`] to the ui
    pub fn page(
        mut self,
        display_name: &'static str,
        file_name: &'static str,
        page: impl SwaggapiPage,
    ) -> Self {
        fn helper<T: SwaggapiPage>(_: T) -> &'static SwaggapiPageBuilder {
            T::builder()
        }
        self.pages.push((display_name, file_name, helper(page)));
        self
    }
}

#[cfg(feature = "actix")]
const _: () = {
    use actix_web::dev::{AppService, HttpServiceFactory};
    use actix_web::web::{scope, Json, Redirect};
    use actix_web::{web, HttpResponse, Responder, Route};

    impl HttpServiceFactory for SwaggerUi {
        fn register(self, app: &mut AppService) {
            let mut config = self.config;
            config.urls.extend(
                self.pages
                    .iter()
                    .map(|&(page_name, file_name, _)| UrlObject::new(page_name, file_name)),
            );
            let config = Arc::new(config);

            let mut scope = scope(self.path)
                .route(
                    "/",
                    serve_static(|| Redirect::to("index.html?configUrl=config.json")),
                )
                .route("config.json", serve_static(move || Json(config)));
            for (_, file_name, builder) in self.pages {
                scope = scope.route(file_name, serve_static(|| Json(builder.build())));
            }
            for file_name in swagger_ui::Assets::iter() {
                if let Some(file_content) = swagger_ui::Assets::get(&file_name) {
                    scope = scope.route(
                        &file_name,
                        serve_static(|| HttpResponse::Ok().body(file_content)),
                    );
                }
            }
            scope.register(app)
        }
    }

    fn serve_static<H: FnOnce() -> R + Clone + 'static, R: Responder + 'static>(
        handler: H,
    ) -> Route {
        web::get().to(move || std::future::ready((handler.clone())()))
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use axum::body::Body;
    use axum::response::{IntoResponse, Redirect, Response};
    use axum::routing::MethodRouter;
    use axum::{Json, Router};

    impl<S> From<SwaggerUi> for Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        fn from(value: SwaggerUi) -> Router<S> {
            let mut config = value.config;
            config.urls.extend(
                value
                    .pages
                    .iter()
                    .map(|&(page_name, file_name, _)| UrlObject::new(page_name, file_name)),
            );
            let config = Arc::new(config);

            let mut router = Router::<S>::new()
                .route(
                    "/swagger-ui/",
                    serve_static(|| Redirect::to("index.html?configUrl=config.json")),
                )
                .route(
                    "/swagger-ui/config.json",
                    serve_static(move || Json(config)),
                );
            for (_, file_name, builder) in value.pages {
                router = router.route(
                    &format!("/swagger-ui/{file_name}"),
                    serve_static(|| Json(builder.build())),
                );
            }
            for file_name in swagger_ui::Assets::iter() {
                if let Some(file_content) = swagger_ui::Assets::get(&file_name) {
                    router = router.route(
                        &format!("/swagger-ui/{file_name}"),
                        serve_static(|| Response::new(Body::from(file_content))),
                    );
                }
            }

            router
        }
    }

    fn serve_static<S, H, R>(handler: H) -> MethodRouter<S>
    where
        S: Clone,
        H: FnOnce() -> R + Clone,
        R: IntoResponse,
        S: Send + Sync + 'static,
        H: Send + Sync + 'static,
        R: Send + 'static,
    {
        MethodRouter::new().get(move || std::future::ready((handler.clone())()))
    }
};
