pub struct COMMAND;
pub struct ADD;
pub struct INIT;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

impl COMMAND {
    pub const ADD: &'static str = "add";
    pub const INIT: &'static str = "init";
}

impl ADD {
    pub const COMPONENTS: &'static str = "components";
    pub const HELP: &'static str = "The components to add (space-separated)";
    pub const ABOUT: &'static str = "Add components and dependencies to your project";
}

impl INIT {
    pub const PROJECT_NAME: &'static str = "project_name";
    pub const HELP: &'static str = "The name of the project to initialize";
    pub const ABOUT: &'static str = "Initialize the project";
}
