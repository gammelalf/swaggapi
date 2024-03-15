use crate::internals::{AccessSwaggapiPageBuilder, SwaggapiPageBuilder};

/// An implicit [`SwaggapiPage`] which will always contain your entire api
pub struct PageOfEverything;
/// "Manual expansion" of [`derive(SwaggapiPage)`](crate::SwaggapiPage)
impl AccessSwaggapiPageBuilder for PageOfEverything {
    fn builder() -> &'static SwaggapiPageBuilder {
        static BUILDER: SwaggapiPageBuilder = SwaggapiPageBuilder::new();
        &BUILDER
    }
}

/// A page is a collection of api endpoints
///
/// You can think of each type implementing this as one `openapi.json`.
///
/// ## Why
///
/// This can be useful if you want to split your api into separate parts with separate openapi files.
///
/// If you don't need this, you can ignore it.
/// The [`PageOfEverything`] will be used implicitly, if you don't say otherwise.
///
/// ## How
///
/// 1. Use [`#[derive(SwaggapiPage)]`](macro@SwaggapiPage) on a unit struct to create a new api page
/// 2. Pass the unit struct to [`ApiContext::page`] to add some endpoints
/// 3. Pass the unit struct to [`SwaggerUi::page`] to expose it in the swagger ui
pub trait SwaggapiPage: AccessSwaggapiPageBuilder {}
impl<P: AccessSwaggapiPageBuilder> SwaggapiPage for P {}
