#[allow(unused)]
pub struct Dependency<'a> {
    pub name: &'a str,
    pub version: Option<&'a str>,
    pub features: &'a [&'a str]
}

impl<'a> Dependency<'a> {
    const fn new(
        name: &'a str,
        version: Option<&'a str>,
        features: &'a [&'a str]
    ) -> Self {
        Dependency { name, version, features }
    }
}


/// 
/// Dependencies to initialize the ui lib
/// 
pub const INIT_DEPENDENCIES: [Dependency<'static>; 2] = [
    Dependency::new(
        "leptos", 
        None, 
        &["csr"]
    ),
    Dependency::new(
        "tw_merge", 
        None, 
        &["variant"]
    )
]; 

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
