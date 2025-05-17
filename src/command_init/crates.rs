pub struct Crate {
    pub name: &'static str,
    #[allow(unused)] // TODO. Find a way to set the version to "0.8" instead of "0.8.2".
    pub version: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl Crate {
    const fn new(name: &'static str, version: Option<&'static str>, features: &'static [&'static str]) -> Self {
        Crate {
            name,
            version,
            features,
        }
    }
}

///
/// Crates to initialize the project.
///
pub const INIT_CRATES: [Crate; 2] = [
    Crate::new("leptos", None, &["csr"]),
    Crate::new("tw_merge", None, &["variant"]),
];
