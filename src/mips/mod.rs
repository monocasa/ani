use super::{Machine, MachineError, Platform};

#[derive(Default)]
struct Sys161Machine;

impl Machine for Sys161Machine {
	fn run(&self) -> Result<(), MachineError> {
		Ok(())
	}
}

pub struct Sys161Platform;

pub const SYS161_PLATFORM: &'static Sys161Platform = &Sys161Platform;

impl Platform for Sys161Platform {
	fn get_short_name(&self) -> &'static str {
		"sys161"
	}

	fn get_long_name(&self) -> &'static str {
		"Harvard MIPS System/161"
	}

	fn get_usage(&self) -> &'static str {
		"CONFIG_FILE KERNEL"
	}

	fn initialize_machine(&self, args: &Vec<String>) -> Result<Box<Machine>, MachineError> {
		if args.len() != 2 {
			return Err(MachineError::InvalidArgs);
		}
		let machine = Box::<Sys161Machine>::new(Default::default());
		Ok(machine)
	}
}

