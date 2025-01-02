//! Collections of some utilities

/// JSON Extractor / Response which doesn't require [`JsonSchema`](schemars::JsonSchema)
///
/// Just think of this type as the `Json<T>` in the framework of your choice
/// and use it if you don't want to bother to make `T` implement [`JsonSchema`](schemars::JsonSchema).
#[derive(Copy, Clone, Debug)]
pub struct SchemalessJson<T>(pub T);

#[cfg(feature = "actix")]
const _: () = {
    use std::future::Future;
    use std::pin::Pin;

    use actix_web::dev::Payload;
    use actix_web::web::Json;
    use actix_web::FromRequest;
    use actix_web::HttpRequest;
    use actix_web::HttpResponse;
    use actix_web::Responder;

    impl<T: 'static> FromRequest for SchemalessJson<T>
    where
        Json<T>: FromRequest,
    {
        type Error = <Json<T> as FromRequest>::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

        fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
            let fut = Json::<T>::from_request(req, payload);
            Box::pin(async move {
                let result = fut.await;
                result.map(|Json(t)| SchemalessJson(t))
            })
        }
    }
    impl<T> Responder for SchemalessJson<T>
    where
        Json<T>: Responder,
    {
        type Body = <Json<T> as Responder>::Body;

        fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
            Json(self.0).respond_to(req)
        }
    }
};

#[cfg(feature = "axum")]
const _: () = {
    use axum::extract::FromRequest;
    use axum::extract::Request;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use axum::Json;

    impl<T, S: Sync> FromRequest<S> for SchemalessJson<T>
    where
        Json<T>: FromRequest<S>,
    {
        type Rejection = <Json<T> as FromRequest<S>>::Rejection;

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            <Json<T> as FromRequest<S>>::from_request(req, state)
                .await
                .map(|Json(t)| SchemalessJson(t))
        }
    }

    impl<T> IntoResponse for SchemalessJson<T>
    where
        Json<T>: IntoResponse,
    {
        fn into_response(self) -> Response {
            Json(self.0).into_response()
        }
    }
};
