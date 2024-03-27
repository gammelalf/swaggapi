use crate::internals::AccessSwaggapiPageBuilder;
use crate::internals::SwaggapiHandler;
use crate::internals::SwaggapiPageBuilder;
use crate::PageOfEverything;
use crate::SwaggapiPage;

/// An `ApiContext` combines several [`SwaggapiHandler`] under a common path.
///
/// It is also responsible for adding them to [`SwaggapiPage`]s once mounted to your application.
pub struct ApiContext<Impl> {
    path: String,
    handlers: Vec<SwaggapiHandler>,
    pages: Vec<&'static SwaggapiPageBuilder>,
    tags: Vec<&'static str>,
    framework_impl: Impl,
}

impl<T> ApiContext<T> {
    fn with_framework_impl(path: String, framework_impl: T) -> Self {
        Self {
            path,
            handlers: Vec::new(),
            pages: Vec::new(),
            tags: Vec::new(),
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

    /// Add a tag to all of this context's handlers
    pub fn tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag);
        self
    }

    fn add_to_pages(&self) {
        for handler in self.handlers.iter().copied() {
            let pages = [PageOfEverything::builder()]
                .into_iter()
                .chain(self.pages.iter().copied());
            for page in pages {
                page.add_handler(
                    format!("{}{}", self.path, handler.path),
                    handler,
                    &self.tags,
                );
            }
        }
    }

    fn map_framework_impl<U>(self, func: impl FnOnce(T) -> U) -> ApiContext<U> {
        let Self {
            path,
            handlers,
            pages,
            tags,
            framework_impl,
        } = self;
        ApiContext {
            path,
            handlers,
            pages,
            tags,
            framework_impl: func(framework_impl),
        }
    }
}

#[cfg(feature = "actix")]
const _: () = {
    use std::future::Future;

    use actix_web::body::MessageBody;
    use actix_web::dev::AppService;
    use actix_web::dev::HttpServiceFactory;
    use actix_web::dev::ServiceFactory;
    use actix_web::dev::ServiceRequest;
    use actix_web::dev::ServiceResponse;
    use actix_web::dev::Transform;
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

    impl<T> ApiContext<Scope<T>>
    where
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        /// Registers a context-wide middleware.
        ///
        /// See [`App::wrap`](actix_web::App::wrap) or [`Scope::wrap`] for more details.
        pub fn wrap<M, B>(
            self,
            middleware: M,
        ) -> ApiContext<
            Scope<
                impl ServiceFactory<
                    ServiceRequest,
                    Config = (),
                    Response = ServiceResponse<B>,
                    Error = actix_web::Error,
                    InitError = (),
                >,
            >,
        >
        where
            M: Transform<
                    T::Service,
                    ServiceRequest,
                    Response = ServiceResponse<B>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
            B: MessageBody,
        {
            self.map_framework_impl(|scope| scope.wrap(middleware))
        }

        /// Registers a context-wide function middleware.
        ///
        /// See [`App::wrap_fn`](actix_web::App::wrap_fn) or [`Scope::wrap_fn`] for more details.
        pub fn wrap_fn<F, R, B>(
            self,
            middleware: F,
        ) -> ApiContext<
            Scope<
                impl ServiceFactory<
                    ServiceRequest,
                    Config = (),
                    Response = ServiceResponse<B>,
                    Error = actix_web::Error,
                    InitError = (),
                >,
            >,
        >
        where
            F: Fn(ServiceRequest, &T::Service) -> R + Clone + 'static,
            R: Future<Output = Result<ServiceResponse<B>, actix_web::Error>>,
            B: MessageBody,
        {
            self.map_framework_impl(|scope| scope.wrap_fn(middleware))
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
    use std::convert::Infallible;

    use axum::extract::Request;
    use axum::response::IntoResponse;
    use axum::routing::MethodRouter;
    use axum::routing::Route;
    use axum::routing::Router;
    use tower::Layer;
    use tower::Service;

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

        /// Apply a [`tower::Layer`] to all routes in the context.
        ///
        /// See [`Router::layer`] for more details.
        pub fn layer<L>(self, layer: L) -> Self
        where
            L: Layer<Route> + Clone + Send + 'static,
            L::Service: Service<Request> + Clone + Send + 'static,
            <L::Service as Service<Request>>::Response: IntoResponse + 'static,
            <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
            <L::Service as Service<Request>>::Future: Send + 'static,
        {
            self.map_framework_impl(|router| router.layer(layer))
        }

        /// Apply a [`tower::Layer`] to the context that will only run if the request matches a route.
        ///
        /// See [`Router::route_layer`] for more details.
        pub fn route_layer<L>(self, layer: L) -> Self
        where
            L: Layer<Route> + Clone + Send + 'static,
            L::Service: Service<Request> + Clone + Send + 'static,
            <L::Service as Service<Request>>::Response: IntoResponse + 'static,
            <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
            <L::Service as Service<Request>>::Future: Send + 'static,
        {
            self.map_framework_impl(|router| router.route_layer(layer))
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
