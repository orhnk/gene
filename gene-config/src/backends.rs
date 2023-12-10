macro_rules! generate_backends {
	($($name:ident),*) => {
		struct Backends {
			$(
				pub $name: bool,
			)*
		}
	};
	($($name:ident),*,) => {
		generate_backends!($($name),*);
	};
    () => {
		compile_error!("This macro requires at least one argument")
	};
}

/// SPM backends
generate_backends!(
	brew,
	choco,
	dpkg,
	flatpak,
	pacman,
	rpm,
	scoop,
	snap,
	zypper,
);

