use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::Handler;

pub struct ApiContext<Impl> {
    path: String,
    handlers: Vec<Handler>,
    framework_impl: Impl,
}

impl<T> ApiContext<T> {
    pub fn handler(mut self, handler: Handler) -> Self {
        self.handlers.push(handler);
        self
    }
}

#[cfg(feature = "actix")]
const _: () = {
    use actix_web::body::MessageBody;
    use actix_web::dev::{
        AppService, HttpServiceFactory, ServiceFactory, ServiceRequest, ServiceResponse, Transform,
    };
    use actix_web::Scope;

    impl ApiContext<Scope> {
        pub fn new(path: &str) -> Self {
            Self {
                path: path.to_string(),
                handlers: Vec::new(),
                framework_impl: Scope::new(path),
            }
        }
    }

    impl<T> HttpServiceFactory for ApiContext<Scope<T>>
    where
        Scope<T>: HttpServiceFactory,
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        fn register(self, config: &mut AppService) {
            let mut scope = self.framework_impl;
            for handler in self.handlers {
                scope = scope.route(handler.path, (handler.actix)());
            }
            scope.register(config)
        }
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use axum::routing::{MethodRouter, Router};

    impl ApiContext<Router> {
        pub fn new(path: &str) -> Self {
            Self {
                path: path.to_string(),
                handlers: Vec::new(),
                framework_impl: Router::new(),
            }
        }
    }

    impl From<ApiContext<Router>> for Router {
        fn from(context: ApiContext<Router>) -> Self {
            let mut routes: BTreeMap<Cow<'static, str>, MethodRouter> = BTreeMap::new();
            for handler in context.handlers {
                let path = if context.path.is_empty() {
                    Cow::Borrowed(handler.path)
                } else {
                    Cow::Owned(format!("{}{}", context.path, handler.path))
                };
                let existing = routes.remove(&path).unwrap_or_default();
                let new = (handler.axum)();
                routes.insert(path, existing.merge(new));
            }

            routes
                .into_iter()
                .fold(context.framework_impl, |router, (path, method_router)| {
                    router.route(&path, method_router)
                })
        }
    }
};
