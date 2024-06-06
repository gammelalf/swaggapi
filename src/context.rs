use crate::internals::SwaggapiHandler;
use crate::internals::SwaggapiPageBuilderImpl;
use crate::internals::{AccessSwaggapiPageBuilder, ContextHandler};
use crate::page::SwaggapiPageBuilder;
use crate::PageOfEverything;
use crate::SwaggapiPage;

/// An `ApiContext` combines several [`SwaggapiHandler`] under a common path.
///
/// It is also responsible for adding them to [`SwaggapiPage`]s once mounted to your application.
#[derive(Debug)]
pub struct ApiContext<Impl> {
    /* The same collection of handlers in swaggapi and framework specific representation */
    /// The contained handlers
    handlers: Vec<ContextHandler>,
    /// The framework implementation of a "router"
    ///
    /// This is a `Router` for axum and a `Scope` for actix.
    framework_impl: Impl,

    /* Parameters added to new handlers */
    /// A base path all handlers are routed under
    ///
    /// This is effectively remembers the argument actix' `Scope` was created with.
    /// Since `Router` doesn't take a path, this will always be empty for axum.
    path: String,

    /// Changes have to be applied to already existing `handlers` manually
    pages: Vec<&'static SwaggapiPageBuilder>,

    /// Changes have to be applied to already existing `handlers` manually
    tags: Vec<&'static str>,
}

impl<T> ApiContext<T> {
    fn with_framework_impl(path: String, framework_impl: T) -> Self {
        Self {
            handlers: Vec::new(),
            framework_impl,

            path,
            pages: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Add a handler to the context
    #[allow(private_bounds)]
    pub fn handler(mut self, handler: SwaggapiHandler) -> Self
    where
        Self: ValidFrameworkImpl,
    {
        self.push_handler(ContextHandler::new(handler));

        ValidFrameworkImpl::handler(self, handler)
    }

    /// Attach a [`SwaggapiPage`] this context's handlers will be added to
    pub fn page(mut self, page: impl SwaggapiPage) -> Self {
        self.pages.push(page.get_builder());
        for handler in &mut self.handlers {
            handler.pages.insert(page.get_builder());
        }
        self
    }

    /// Add a tag to all of this context's handlers
    pub fn tag(mut self, tag: &'static str) -> Self {
        self.tags.push(tag);
        for handler in &mut self.handlers {
            handler.tags.insert(tag);
        }
        self
    }

    /// Adds a [`ContextHandler`] after adding this context's `path`, `tags` and `pages` to it
    fn push_handler(&mut self, mut handler: ContextHandler) {
        if !self.path.is_empty() {
            handler.path = format!("{}{}", self.path, handler.path);
        }
        handler.tags.extend(self.tags.iter().copied());
        handler.pages.extend(self.pages.iter().copied());
        self.handlers.push(handler);
    }

    /// Adds the handlers to their api pages and returns the contained framework impl
    fn finish(self) -> T {
        for mut handler in self.handlers {
            handler.path = framework_path_to_openapi(handler.path);

            SwaggapiPageBuilderImpl::add_handler(PageOfEverything.get_builder(), &handler);
            for page in handler.pages.iter() {
                SwaggapiPageBuilderImpl::add_handler(page, &handler);
            }
        }
        return self.framework_impl;

        /// Converts the framework's syntax for path parameters into openapi's

        fn framework_path_to_openapi(framework_path: String) -> String {
            #[cfg(feature = "axum")]
            {
                use std::borrow::Cow;
                use std::sync::OnceLock;

                use regex::Regex;

                static RE: OnceLock<Regex> = OnceLock::new();

                let regex = RE.get_or_init(|| Regex::new(":([^/]*)").unwrap());
                match regex.replace_all(&framework_path, "{$1}") {
                    Cow::Borrowed(_) => framework_path,
                    Cow::Owned(new_path) => new_path,
                }
            }

            #[cfg(not(feature = "axum"))]
            framework_path
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

/// Helper trait to have framework independent methods
/// use framework specific implementations
trait ValidFrameworkImpl {
    fn handler(self, handler: SwaggapiHandler) -> Self;
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
        /// Adds a sub context
        ///
        /// See [`Scope::service`] (called with another `Scope`).
        pub fn service(mut self, other: ApiContext<impl HttpServiceFactory + 'static>) -> Self {
            for handler in other.handlers {
                self.push_handler(handler);
            }
            self.map_framework_impl(|x| x.service(other.framework_impl))
        }

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

    impl<T> ValidFrameworkImpl for ApiContext<Scope<T>>
    where
        Scope<T>: HttpServiceFactory,
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        fn handler(self, handler: SwaggapiHandler) -> Self {
            self.map_framework_impl(|x| x.route(&handler.path, (handler.actix)()))
        }
    }

    impl<T> HttpServiceFactory for ApiContext<Scope<T>>
    where
        Scope<T>: HttpServiceFactory,
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        fn register(self, config: &mut AppService) {
            self.finish().register(config)
        }
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use std::convert::Infallible;

    use axum::extract::Request;
    use axum::response::IntoResponse;
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
        pub fn new() -> Self {
            Self::with_framework_impl(String::new(), Router::new())
        }

        /// Create a new context with a tag
        ///
        /// (Shorthand for `ApiContext::new().tag(...)`)
        pub fn with_tag(tag: &'static str) -> Self {
            Self::new().tag(tag)
        }

        /// Calls [`Router::nest`] while preserving api information
        pub fn nest(mut self, path: &str, other: ApiContext<Router>) -> Self {
            for mut handler in other.handlers {
                // Code taken from `path_for_nested_route` in `axum/src/routing/path_router.rs`
                handler.path = if path.ends_with('/') {
                    format!("{path}{}", handler.path.trim_start_matches('/'))
                } else if handler.path == "/" {
                    path.into()
                } else {
                    format!("{path}{}", handler.path)
                };

                self.push_handler(handler);
            }
            self.map_framework_impl(|x| x.nest(path, other.framework_impl))
        }

        /// Calls [`Router::merge`] while preserving api information
        pub fn merge(mut self, other: ApiContext<Router>) -> Self {
            for handler in other.handlers {
                self.push_handler(handler);
            }
            self.map_framework_impl(|x| x.merge(other.framework_impl))
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

    impl ValidFrameworkImpl for ApiContext<Router> {
        fn handler(self, handler: SwaggapiHandler) -> Self {
            self.map_framework_impl(|x| x.route(&handler.path, (handler.axum)()))
        }
    }

    impl From<ApiContext<Router>> for Router {
        fn from(context: ApiContext<Router>) -> Self {
            context.finish()
        }
    }
};
