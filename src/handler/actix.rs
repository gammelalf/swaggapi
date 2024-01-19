use crate::handler::Handler;
#[macro_export]
#[doc(hidden)]
macro_rules! impl_Foo_actix {
    ($module:ident::$ident:ident: fn($($arg:ty),*) -> $ret:ty) => {
        fn actix(&self) -> $crate::re_exports::actix_web::Route {
            impl $crate::re_exports::actix_web::dev::HttpServiceFactory for $ident {
                fn register(self, config: &mut $crate::re_exports::actix_web::dev::AppService) {
                    $crate::re_exports::actix_web::Resource::new(self.path())
                        .route(self.actix())
                        .register(config)
                }
            }

            <$crate::PageOfEverything as $crate::SwaggapiPage>::builder().add_handler(self);
            $crate::re_exports::actix_web::Route::new()
                .method(self.method().actix())
                .to($module::$ident)
        }
    };
}
