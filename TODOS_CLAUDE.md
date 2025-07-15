

- CliError::file_operation("Failed to get parent directory")
  - should not pass str as param but just have the ffuction without str param




;li/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*


- whenever possible use error built in iintead of custom CliError




;l/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:



remove and use directly :

pub struct MyCommand;
pub struct AddCommand;
pub struct InitCommand;
pub struct StartersCommand;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

impl MyCommand {
    pub const ADD: &'static str = "add";
    pub const INIT: &'static str = "init";
    pub const STARTERS: &'static str = "starters";
}

impl AddCommand {
    pub const COMPONENTS: &'static str = "components";
    pub const HELP: &'static str = "The components to add (space-separated)";
    pub const ABOUT: &'static str = "Add components and dependencies to your project";
}

impl InitCommand {
    pub const PROJECT_NAME: &'static str = "project_name";
    pub const HELP: &'static str = "The name of the project to initialize";
    pub const ABOUT: &'static str = "Initialize the project";
}

impl StartersCommand {
    pub const ABOUT: &'static str = "Choose and install starter templates";
}




