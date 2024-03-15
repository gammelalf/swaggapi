use std::fmt;

/// Framework independent enum of HTTP methods
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum HttpMethod {
    /// `GET`
    Get,
    /// `POST`
    Post,
    /// `PUT`
    Put,
    /// `DELETE`
    Delete,
    /// `HEAD`
    Head,
    /// `OPTIONS`
    Options,
    /// `PATCH`
    Patch,
    /// `TRACE`
    Trace,
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Trace => "TRACE",
        })
    }
}
impl HttpMethod {
    /// Convert into actix's [`Method`](actix_web::http::Method)
    #[cfg(feature = "actix")]
    pub fn actix(&self) -> actix_web::http::Method {
        use actix_web::http::Method as Actix;
        match self {
            HttpMethod::Get => Actix::GET,
            HttpMethod::Post => Actix::POST,
            HttpMethod::Put => Actix::PUT,
            HttpMethod::Delete => Actix::DELETE,
            HttpMethod::Head => Actix::HEAD,
            HttpMethod::Options => Actix::OPTIONS,
            HttpMethod::Patch => Actix::PATCH,
            HttpMethod::Trace => Actix::TRACE,
        }
    }

    /// Convert into axum's [`MethodFilter`](axum::routing::MethodFilter)
    #[cfg(feature = "axum")]
    pub fn axum(&self) -> axum::routing::MethodFilter {
        use axum::routing::MethodFilter as Axum;
        match self {
            HttpMethod::Get => Axum::GET,
            HttpMethod::Post => Axum::POST,
            HttpMethod::Put => Axum::PUT,
            HttpMethod::Delete => Axum::DELETE,
            HttpMethod::Head => Axum::HEAD,
            HttpMethod::Options => Axum::OPTIONS,
            HttpMethod::Patch => Axum::PATCH,
            HttpMethod::Trace => Axum::TRACE,
        }
    }
}
