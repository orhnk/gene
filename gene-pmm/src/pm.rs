use std::mem::size_of_val;

pub struct PackageManagerArgs<'a> {
    package_manager: &'a str,
    args: &'a [&'a str],
}

/// List of supported package manager backends
pub enum PackageManager {
    APT,
    Pacman,
}

impl ToString for PackageManager {
    fn to_string(&self) -> String {
        use PackageManager::*;

        let pm_name = match self {
            APT => "apt",
            Pacman => "pacman",
        };

        pm_name.to_string()
    }
}

impl<'a> PackageManagerArgs<'a> {
    pub fn new(package_manager: &'a str, args: &'a [&'a str]) -> Self {
        Self {
            package_manager,
            args,
        }
    }
}

impl ToString for PackageManagerArgs<'_> {
    fn to_string(&self) -> String {
        let mut cmd: Vec<&str> = Vec::with_capacity(
            size_of_val(self.args) + size_of_val(self.package_manager) + self.args.len(), // allocate enough memory to fit package manager name, arguments and spaces between both of them.
        );

        cmd.push(self.package_manager);
        cmd.extend_from_slice(self.args);

        cmd.join(" ")
    }
}
