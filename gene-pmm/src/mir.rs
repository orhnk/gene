use crate::pm::PackageManager;

/// List of actions that GENE can infer
#[derive(Debug, Clone, Copy)]
pub enum GeneMIRActions {
    Install,
    Remove,
    Query,
    Update,
}

/// GENE Middle Intermediate Representation
///
/// Abstraction to make it easier to transpile between different package managers.
/// Just like how LLVM IR works but dead simple.
pub struct GeneMIR<'a> {
    /// Package Manager backend to use
    package_manager: PackageManager,
    /// Options that get transpiled
    actions: Vec<GeneMIRActions>,
    /// Non-transpiled Options
    raw_actions: Vec<&'a str>,
}

impl<'a> GeneMIR<'a> {
    pub fn new(package_manager: PackageManager) -> Self {
        GeneMIR {
            package_manager,
            actions: Default::default(),
            raw_actions: Default::default(),
        }
    }
    pub fn actions(mut self, actions: Vec<GeneMIRActions>) -> Self {
        self.actions.extend(actions);
        self
    }

    pub fn raw_actions(mut self, raw_actions: Vec<&'a str>) -> Self {
        self.raw_actions.extend(raw_actions);
        self
    }
}
