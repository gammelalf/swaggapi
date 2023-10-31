use openapiv3::{Operation, Parameter, ReferenceOr, RequestBody, Responses};

/// Meta information about a handler gathered by the [`#[operation]`](operation) macro
#[derive(Default)]
pub struct OperationDescription {
    /// `true` if `#[deprecated]` is present
    pub deprecated: bool,

    /// Set by macro if `#[doc = "..."]` (i.e. a doc comment) is present
    pub doc: &'static [&'static str],

    /// The handler's identifier
    pub ident: &'static str,

    pub responses: Responses,

    pub request_body: Vec<RequestBody>,

    pub parameters: Vec<Parameter>,
}

impl OperationDescription {
    pub fn build(mut self) -> Operation {
        let mut doc = self.doc.into_iter();
        let summary = doc.next().map(|line| line.trim().to_string());
        let description = summary
            .clone()
            .map(|summary| doc.fold(summary, |text, line| format!("{text}\n{}", line.trim())));

        Operation {
            summary,
            description,
            operation_id: Some(self.ident.to_string()),
            parameters: self.parameters.into_iter().map(ReferenceOr::Item).collect(),
            request_body: self.request_body.pop().map(ReferenceOr::Item),
            responses: self.responses,
            deprecated: self.deprecated,
            security: None,   // TODO
            tags: Vec::new(), // TODO
            // Not supported:
            external_docs: Default::default(),
            servers: Default::default(),
            extensions: Default::default(),
        }
    }
}
