pub struct Dependency<'a> {
    pub name: &'a str,
    #[allow(unused)] // TODO. Find a way to set the version to "0.8" instead of "0.8.2".
    pub version: Option<&'a str>,
    pub features: &'a [&'a str],
}

impl<'a> Dependency<'a> {
    const fn new(name: &'a str, version: Option<&'a str>, features: &'a [&'a str]) -> Self {
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
pub const INIT_DEPENDENCIES: [Dependency<'static>; 2] = [
    Dependency::new("leptos", None, &["csr"]),
    Dependency::new("tw_merge", None, &["variant"]),
];
