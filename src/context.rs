use crate::internals::AccessSwaggapiPageBuilder;
use crate::internals::{SwaggapiHandler, SwaggapiPageBuilder};
use crate::{PageOfEverything, SwaggapiPage};

/// An `ApiContext` combines several [`SwaggapiHandler`] under a common path.
///
/// It is also responsible for adding them to [`SwaggapiPage`]s once mounted to your application.
pub struct ApiContext<Impl> {
    path: String,
    handlers: Vec<SwaggapiHandler>,
    pages: Vec<&'static SwaggapiPageBuilder>,
    framework_impl: Impl,
}

impl<T> ApiContext<T> {
    fn with_framework_impl(path: String, framework_impl: T) -> Self {
        Self {
            path,
            handlers: Vec::new(),
            pages: Vec::new(),
            framework_impl,
        }
    }

    /// Add a handler to the context
    ///
    /// The handler will be routed under the context's path i.e. `"{ctx_path}/{handler_path}"`.
    pub fn handler(mut self, handler: SwaggapiHandler) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Attach a [`SwaggapiPage`] this context's handlers will be added to
    pub fn page<Page: SwaggapiPage>(mut self, _: Page) -> Self {
        self.pages.push(Page::builder());
        self
    }

    fn add_to_pages(&self) {
        for handler in self.handlers.iter().copied() {
            let pages = [PageOfEverything::builder()]
                .into_iter()
                .chain(self.pages.iter().copied());
            for page in pages {
                page.add_handler(format!("{}{}", self.path, handler.path), handler);
            }
        }
    }
}

#[cfg(feature = "actix")]
const _: () = {
    use actix_web::dev::{AppService, HttpServiceFactory, ServiceFactory, ServiceRequest};
    use actix_web::Scope;

    impl ApiContext<Scope> {
        /// Create a new context
        ///
        /// It wraps an actix [`Scope`] internally and should be added to your application using [`App::service`](actix_web::App::service):
        /// ```rust
        /// # use actix_web::App;
        /// # use swaggapi::ApiContext;
        /// let app = App::new().service(ApiContext::new("/api"));
        /// ```
        pub fn new(path: &str) -> Self {
            Self::with_framework_impl(path.to_string(), Scope::new(path))
        }
    }

    impl<T> HttpServiceFactory for ApiContext<Scope<T>>
    where
        Scope<T>: HttpServiceFactory,
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        fn register(self, config: &mut AppService) {
            self.add_to_pages();

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
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    use axum::routing::{MethodRouter, Router};

    impl ApiContext<Router> {
        /// Create a new context
        ///
        /// It wraps an axum [`Router`] internally and should be added to your application's router using [`Router::merge`]:
        /// ```rust
        /// # use axum::Router;
        /// # use swaggapi::ApiContext;
        /// let app = Router::new().merge(ApiContext::new("/api"));
        /// ```
        pub fn new(path: &str) -> Self {
            Self::with_framework_impl(path.to_string(), Router::new())
        }
    }

    impl From<ApiContext<Router>> for Router {
        fn from(context: ApiContext<Router>) -> Self {
            context.add_to_pages();

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
