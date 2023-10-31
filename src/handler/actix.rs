use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::Resource;

use crate::handler::Handler;
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($ident:path: fn($($arg:ty),*) -> $ret:ty) => {
        fn actix(&self) -> $crate::re_exports::actix_web::Route {
            $crate::re_exports::actix_web::Route::new()
                .method(self.method().actix())
                .to($ident)
        }
    };
}

impl HttpServiceFactory for &dyn Handler {
    fn register(self, config: &mut AppService) {
        Resource::new(self.path())
            .route(self.actix())
            .register(config)
    }
}
