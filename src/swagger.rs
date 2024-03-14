use std::borrow::Cow;

pub fn file(name: &str) -> Option<Cow<'static, [u8]>> {
    if name == "swagger-initializer.js" {
        let config = serde_json::to_string(&serde_json::json!(
            {
                "url": "/openapi",
                "dom_id": "#swagger-ui",
                "deepLinking": true,
                "layout": "StandaloneLayout"
              }
        ))
        .unwrap();
        Some(Cow::Owned(
            format!(r#"window.onload = function() {{ const config = {config}; window.ui = SwaggerUIBundle({{presets: [ SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset ], plugins: [ SwaggerUIBundle.plugins.DownloadUrl ], ...config}}); }};"#)
                .into_bytes(),
        ))
    } else {
        file_impl(if name.is_empty() || name == "/" {
            "index.html"
        } else {
            name
        })
        .map(Cow::Borrowed)
    }
}

macro_rules! gen_file_impl {
    ($($file:literal,)*) => {
        #[allow(dead_code)]
        fn file_impl(name: &str) -> Option<&'static [u8]> {
            match name {
                $($file => Some(include_bytes!(concat!("../swagger-dist/", $file))),)*
                _ => None,
            }
        }
    };
}
gen_file_impl![
    "index.html",
    "swagger-ui.css",
    "index.css",
    "favicon-32x32.png",
    "favicon-16x16.png",
    "swagger-ui-bundle.js",
    "swagger-ui-standalone-preset.js",
];
