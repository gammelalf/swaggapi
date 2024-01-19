use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::Resource;

use crate::Handler;
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($module:ident::$ident:ident: fn($($arg:ty),*) -> $ret:ty) => {
        || {
            <$crate::PageOfEverything as $crate::SwaggapiPage>::builder().add_handler(&$ident);
            $crate::re_exports::actix_web::Route::new()
                .method($ident.method.actix())
                .to($module::$ident)
        }
    };
}

impl HttpServiceFactory for Handler {
    fn register(self, config: &mut AppService) {
        Resource::new(self.path)
            .route((self.actix)())
            .register(config)
    }
}
