use std::sync::Arc;
use std::sync::Mutex;

use openapiv3::OpenAPI;

use crate::internals::AccessSwaggapiPageBuilder;
use crate::internals::SwaggapiPageBuilderImpl;

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
pub trait SwaggapiPage: AccessSwaggapiPageBuilder {
    /// Returns the [`OpenAPI`] file
    ///
    /// The internal build process is cached (hence the `Arc`) so feel free to call this eagerly.
    fn openapi(&self) -> Arc<OpenAPI>;
}
impl<P: AccessSwaggapiPageBuilder> SwaggapiPage for P {
    fn openapi(&self) -> Arc<OpenAPI> {
        SwaggapiPageBuilderImpl::build(self.get_builder())
    }
}

/// An implicit [`SwaggapiPage`] which will always contain your entire api
pub struct PageOfEverything;
/// "Manual expansion" of [`derive(SwaggapiPage)`](crate::SwaggapiPage)
impl AccessSwaggapiPageBuilder for PageOfEverything {
    fn get_builder(&self) -> &'static SwaggapiPageBuilder {
        static BUILDER: SwaggapiPageBuilder = SwaggapiPageBuilder::new();
        &BUILDER
    }
}

/// Collection of openapi paths and schemas
///
/// This struct is intended to be used through a `&'static` provided
/// by an [`AccessSwaggapiPageBuilder`] type.
pub struct SwaggapiPageBuilder {
    pub(crate) title: &'static str,
    pub(crate) version: &'static str,
    pub(crate) state: Mutex<Option<SwaggapiPageBuilderImpl>>,
}

impl SwaggapiPageBuilder {
    /// Construct a new empty builder
    ///
    /// Builders will be stored in `static` variables, so this function has to be `const`.
    pub const fn new() -> Self {
        Self {
            title: "",
            version: "",
            state: Mutex::new(None),
        }
    }
}
