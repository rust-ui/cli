pub struct Dependency {
    pub name: &'static str,
    #[allow(unused)] // TODO. Find a way to set the version to "0.8" instead of "0.8.2".
    pub version: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl Dependency {
    const fn new(name: &'static str, version: Option<&'static str>, features: &'static [&'static str]) -> Self {
        Dependency {
            name,
            version,
            features,
        }
    }
}

///
/// Dependencies to initialize the ui lib
///
pub const INIT_DEPENDENCIES: [Dependency; 2] = [
    Dependency::new("leptos", None, &["csr"]),
    Dependency::new("tw_merge", None, &["variant"]),
];
