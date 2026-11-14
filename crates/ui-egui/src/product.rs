//! The product an app built on this interface presents itself as (DexPdfCraft fork).
//!
//! An app that ships PdfCraft's interface under its own name sets a [`Product`] once at start-up;
//! About then shows that name, version and text first, and PdfCraft's own details below, as what
//! the app is built on. Without one, About is PdfCraft's, as upstream.

use std::sync::OnceLock;

/// An app built on PdfCraft's interface.
#[derive(Clone, Debug)]
pub struct Product {
    pub name: &'static str,
    pub version: &'static str,
    /// About's text, by interface language code ("en" is the fallback): paragraphs.
    pub about: Vec<(&'static str, Vec<String>)>,
    /// Links shown under the text: (label, URL).
    pub links: Vec<(String, String)>,
    /// Whether the title bar shows PdfCraft's Discord button. An app under its own name has no
    /// PdfCraft community of its own to send people to from there (Help ▸ Discord stays).
    pub discord_button: bool,
}

impl Product {
    /// The paragraphs for interface language `code`, else English.
    pub fn about_in(&self, code: &str) -> &[String] {
        self.about
            .iter()
            .find(|(c, _)| *c == code)
            .or_else(|| self.about.iter().find(|(c, _)| *c == "en"))
            .map_or(&[], |(_, p)| p.as_slice())
    }
}

static PRODUCT: OnceLock<Product> = OnceLock::new();

/// Present the interface as `product` (once; later calls are ignored).
pub fn set(product: Product) {
    let _ = PRODUCT.set(product);
}

pub fn get() -> Option<&'static Product> {
    PRODUCT.get()
}

/// Whether the title bar shows the Discord button: yes, unless the product says otherwise.
pub fn discord_button() -> bool {
    get().is_none_or(|p| p.discord_button)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_falls_back_to_english() {
        let p = Product {
            name: "X",
            version: "1",
            about: vec![("en", vec!["Hello".into()]), ("es", vec!["Hola".into()])],
            links: Vec::new(),
            discord_button: false,
        };
        assert_eq!(p.about_in("es"), ["Hola".to_owned()]);
        assert_eq!(p.about_in("ja"), ["Hello".to_owned()]);
    }
}
