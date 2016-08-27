use super::{Machine, MachineError, Platform};

#[derive(Default)]
struct MaltaMachine;

impl Machine for MaltaMachine {
	fn run(&self) -> Result<(), MachineError> {
		Ok(())
	}
}

pub struct MaltaPlatform;

pub const MALTA_PLATFORM: &'static MaltaPlatform = &MaltaPlatform;

impl Platform for MaltaPlatform {
	fn get_short_name(&self) -> &'static str {
		"malta"
	}

	fn get_long_name(&self) -> &'static str {
		"MIPS Malta Core LV"
	}

	fn get_usage(&self) -> &'static str {
		"KERNEL"
	}

	fn initialize_machine(&self, args: &Vec<String>) -> Result<Box<Machine>, MachineError> {
		if args.len() != 1 {
			return Err(MachineError::InvalidArgs);
		}
		let machine = Box::<MaltaMachine>::new(Default::default());
		Ok(machine)
	}
}

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

