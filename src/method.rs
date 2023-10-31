use std::fmt;

/// Http request method
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Method {
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
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Patch => "PATCH",
            Method::Trace => "TRACE",
        })
    }
}
impl Method {
    /// Convert into actix's [`Method`](actix_web::http::Method)
    #[cfg(feature = "actix")]
    pub fn actix(&self) -> actix_web::http::Method {
        use actix_web::http::Method as Actix;
        match self {
            Method::Get => Actix::GET,
            Method::Post => Actix::POST,
            Method::Put => Actix::PUT,
            Method::Delete => Actix::DELETE,
            Method::Head => Actix::HEAD,
            Method::Options => Actix::OPTIONS,
            Method::Patch => Actix::PATCH,
            Method::Trace => Actix::TRACE,
        }
    }

    /// Convert into axum's [`MethodFilter`](axum::routing::MethodFilter)
    #[cfg(feature = "axum")]
    pub fn axum(&self) -> axum::routing::MethodFilter {
        use axum::routing::MethodFilter as Axum;
        match self {
            Method::Get => Axum::GET,
            Method::Post => Axum::POST,
            Method::Put => Axum::PUT,
            Method::Delete => Axum::DELETE,
            Method::Head => Axum::HEAD,
            Method::Options => Axum::OPTIONS,
            Method::Patch => Axum::PATCH,
            Method::Trace => Axum::TRACE,
        }
    }
}
