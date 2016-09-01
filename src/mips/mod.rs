use super::{Machine, MachineError, Platform};

use ani_core::{Arch, CpuCookie, CPU_ENDIAN_BIG, mips, PROT_ALL, System};

struct MaltaMachine {
	system: System,
	cpu: CpuCookie,
}

impl MaltaMachine {
	fn new(system: System, cpu: CpuCookie) -> MaltaMachine {
		MaltaMachine {
			system: system,
			cpu: cpu,
		}
	}
}

impl Machine for MaltaMachine {
	fn run(&mut self) -> Result<(), MachineError> {
		let _ = try!(self.system.execute(&self.cpu));

		Ok(())
	}
}

pub struct MaltaPlatform;

pub const MALTA_PLATFORM: &'static MaltaPlatform = &MaltaPlatform;

const MALTA_RAM_BASE: u64 = 0;
const MALTA_DEFAULT_RAM_SIZE: u64 = 128 * 1024 * 1024;

impl Platform for MaltaPlatform {
	fn get_short_name(&self) -> &'static str {
		"malta"
	}

	fn get_long_name(&self) -> &'static str {
		"MIPS Malta Core LV"
	}

	fn get_usage(&self) -> &'static str {
		"KERNEL KERNEL_CMD_LINE"
	}

	fn initialize_machine(&self, args: &Vec<String>) -> Result<Box<Machine>, MachineError> {
		if args.len() < 3 {
			return Err(MachineError::InvalidArgs);
		}

		let mut system = System::new();

		try!(system.add_mappable_range(PROT_ALL, MALTA_RAM_BASE, MALTA_DEFAULT_RAM_SIZE));

		let cpu = try!(system.register_cpu(CPU_ENDIAN_BIG, Arch::Mips(mips::Arch::R2000)));

		let machine = Box::<MaltaMachine>::new(MaltaMachine::new(system, cpu));
		Ok(machine)
	}
}

#[derive(Default)]
struct Sys161Machine;

impl Machine for Sys161Machine {
	fn run(&mut self) -> Result<(), MachineError> {
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

