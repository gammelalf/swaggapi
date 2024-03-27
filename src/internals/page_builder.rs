use std::collections::BTreeMap;
use std::sync::Arc;

use indexmap::IndexMap;
use openapiv3::Components;
use openapiv3::Info;
use openapiv3::OpenAPI;
use openapiv3::Operation;
use openapiv3::PathItem;
use openapiv3::Paths;
use openapiv3::ReferenceOr;
use schemars::schema::Schema;

use crate::internals::convert_schema;
use crate::internals::HttpMethod;
use crate::internals::SchemaGenerator;
use crate::internals::SwaggapiHandler;
use crate::page::SwaggapiPageBuilder;

/// This trait associates one static instance of a [`SwaggapiPageBuilder`] to its implementor.
///
/// It is the implementation detail of [`SwaggapiPage`](trait@crate::SwaggapiPage)
/// and should be implemented through [`#[derive(SwaggapiPage)]`](macro@crate::SwaggapiPage).
pub trait AccessSwaggapiPageBuilder {
    /// Get the [`SwaggapiPageBuilder`]
    ///
    /// This method will always (using [`#[derive(SwaggapiPage)]`](macro@crate::SwaggapiPage)) be implemented as:
    /// ```rust
    /// # use swaggapi::internals::SwaggapiPageBuilder;
    /// fn builder() -> &'static SwaggapiPageBuilder {
    ///     static BUILDER: SwaggapiPageBuilder = SwaggapiPageBuilder::new();
    ///     &BUILDER
    /// }
    /// ```
    fn builder() -> &'static SwaggapiPageBuilder;

    /// Syntactic sugar
    ///
    /// This method allows function like this:
    /// ```rust
    /// use swaggapi::SwaggapiPage;
    /// fn use_page(page: impl SwaggapiPage) {
    ///     page.get_builder();
    /// }
    /// ```
    ///
    /// Instead of this:
    /// ```rust
    /// use swaggapi::SwaggapiPage;
    /// fn use_page<P: SwaggapiPage>() {
    ///     P::builder();
    /// }
    /// ```
    fn get_builder(&self) -> &'static SwaggapiPageBuilder {
        Self::builder()
    }
}

#[derive(Default)]
pub struct SwaggapiPageBuilderImpl {
    paths: Paths,

    schemas: BTreeMap<String, Schema>,

    /// Cache for the result of [`SwaggapiPageBuilder::build`]
    last_build: Option<Arc<OpenAPI>>,
}

impl SwaggapiPageBuilderImpl {
    /// Add a handler to this api page
    ///
    /// The handler will be registered under a custom `handler_path` instead of using the `handler.path`.
    /// This allows an [`ApiContext`] to modify the path.
    pub fn add_handler(
        builder: &SwaggapiPageBuilder,
        handler_path: String,
        handler: SwaggapiHandler,
        tags: &[&'static str],
    ) {
        let mut guard = builder.state.lock().unwrap();
        let state = guard.get_or_insert_with(Default::default);
        state.last_build = None;

        let (parameters, mut request_body, responses) =
            SchemaGenerator::employ(&mut state.schemas, |gen| {
                let mut parameters = Vec::new();
                let mut request_body = Vec::new();
                for arg in handler.handler_arguments {
                    if let Some(arg) = arg.as_ref() {
                        parameters.extend(
                            (arg.parameters)(&mut *gen)
                                .into_iter()
                                .map(ReferenceOr::Item),
                        );
                        request_body.extend((arg.request_body)(&mut *gen));
                    }
                }
                let responses = (handler.responses)(&mut *gen);
                (parameters, request_body, responses)
            });

        let summary = handler.doc.get(0).map(|line| line.trim().to_string());
        let description = summary.clone().map(|summary| {
            handler
                .doc
                .get(1..)
                .unwrap_or(&[])
                .iter()
                .fold(summary, |text, line| format!("{text}\n{}", line.trim()))
        });

        let operation = Operation {
            summary,
            description,
            operation_id: Some(handler.ident.to_string()),
            parameters,
            request_body: request_body.pop().map(ReferenceOr::Item),
            responses,
            deprecated: handler.deprecated,
            security: None, // TODO
            tags: tags.iter().copied().map(String::from).collect(),
            // Not supported:
            external_docs: Default::default(),
            servers: Default::default(),
            extensions: Default::default(),
            callbacks: Default::default(),
        };

        let ReferenceOr::Item(path) = state
            .paths
            .paths
            .entry(handler_path)
            .or_insert_with(|| ReferenceOr::Item(PathItem::default()))
        else {
            unreachable!("We only ever insert ReferenceOr::Item. See above")
        };
        let operation_mut = match handler.method {
            HttpMethod::Get => &mut path.get,
            HttpMethod::Post => &mut path.post,
            HttpMethod::Put => &mut path.put,
            HttpMethod::Delete => &mut path.delete,
            HttpMethod::Head => &mut path.head,
            HttpMethod::Options => &mut path.options,
            HttpMethod::Patch => &mut path.patch,
            HttpMethod::Trace => &mut path.trace,
        };
        *operation_mut = Some(operation);
    }

    /// Returns the [`OpenAPI`] file
    ///
    /// The build operation is cached (hence the `Arc`) so feel free to call this eagerly.
    pub fn build(builder: &SwaggapiPageBuilder) -> Arc<OpenAPI> {
        let mut guard = builder.state.lock().unwrap();
        let state = guard.get_or_insert_with(Default::default);

        if let Some(open_api) = state.last_build.clone() {
            return open_api;
        }

        let open_api = Arc::new(OpenAPI {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: builder.title.to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                version: builder.version.to_string(),
                extensions: IndexMap::new(),
            },
            servers: vec![],
            paths: state.paths.clone(),
            components: Some(Components {
                schemas: state
                    .schemas
                    .iter()
                    .map(|(key, schema)| (key.clone(), convert_schema(schema.clone())))
                    .collect(),
                ..Default::default()
            }),
            security: None,
            tags: vec![],
            external_docs: None,
            extensions: IndexMap::new(),
        });

        state.last_build = Some(open_api.clone());
        open_api
    }
}
