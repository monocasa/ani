extern crate getopts;

pub mod mips;

use getopts::Options;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

enum ParseError {
	Help,
	Ver,
	NoPlatform,
	PrintPlatforms,
	GetoptErr(getopts::Fail),
	UnknownArgument(&'static str, String),
}

impl From<getopts::Fail> for ParseError {
	fn from(err: getopts::Fail) -> ParseError {
		ParseError::GetoptErr(err)
	}
}

pub enum MachineError {
	Io(std::io::Error),
	InvalidArgs,
}

impl From<std::io::Error> for MachineError {
	fn from(err: std::io::Error) -> MachineError {
		MachineError::Io(err)
	}
}

pub trait Machine {
	fn run(&self) -> Result<(), MachineError>;
}

pub trait Platform {
	fn get_short_name(&self) -> &'static str;
	fn get_long_name(&self) -> &'static str;
	fn get_usage(&self) -> &'static str;
	fn initialize_machine(&self, args: &Vec<String>) -> Result<Box<Machine>, MachineError>;
}

const PLATFORMS: [&'static Platform; 2] = [
	mips::MALTA_PLATFORM,
	mips::SYS161_PLATFORM,
];

type ParseResult = Result<(Vec<String>, AniOptions), ParseError>;

#[derive(Default)]
struct AniOptions {
	platform: Option<&'static Platform>,
}

fn find_platform_by_name(name: &str) -> Option<&'static Platform> {
	for &platform in PLATFORMS.iter() {
		if name == platform.get_short_name() {
			return Some(platform);
		}
	}

	None
}

fn validate_options(ani_options: &AniOptions) -> Result<(), ParseError> {
	match ani_options.platform {
		Some(_) => { },
		None => return Err(ParseError::NoPlatform),
	}

	Ok(())
}

fn parse_options(args: &Vec<String>, opts: &mut Options) -> ParseResult {
	let mut ani_opts: AniOptions = Default::default();

	opts.optflag("H", "help",    "Display this information");
	opts.optflag("v", "version", "Display the version number of ani");

	let matches = try!(opts.parse(&args[1..]));
	let remainder = matches.free.clone();

	match matches.free.first() {
		Some(platform_name) => {
			ani_opts.platform = match platform_name.as_ref() {
				"?" => return Err(ParseError::PrintPlatforms),
				_   => {
					match find_platform_by_name(platform_name) {
						Some(ret_platform) => Some(ret_platform),
						None => return Err(ParseError::UnknownArgument("PLATFORM", platform_name.clone())),
					}
				},
			};
		},
		None => {
			return Err(ParseError::NoPlatform);
		},
	}

	if matches.opt_present("help") {
		return Err(ParseError::Help);
	}

	if matches.opt_present("version") {
		return Err(ParseError::Ver);
	}

	try!(validate_options(&ani_opts));

	Ok((remainder, ani_opts))
}

fn print_usage(program_name: &str) {
	println!("Usage:  {} PLATFORM", program_name);
}

fn on_parse_err(err: ParseError, program_name: &str) {
	let ret_code = match err {
		ParseError::Help => {
			print_usage(program_name);
			0
		},

		ParseError::Ver => {
			println!("ani multi-system emulator {}", VERSION);
			println!("Copyrgiht 2015-2016 Tristan Miller");
			println!("This program is free software; you may redistribute it under the terms of");
			println!("the GNU General Public License version 3 or (at your option) any later version.");
			println!("This program has absolutely no warranty.");
			0
		},

		ParseError::NoPlatform => {
			println!("ERROR:  No platform specified");
			1
		},

		ParseError::PrintPlatforms => {
			println!("Platforms:");
			for platform in PLATFORMS.iter() {
				println!("{:7} - {}", platform.get_short_name(), platform.get_long_name());
			}
			0
		},

		ParseError::GetoptErr(err) => {
			println!("ERROR: {}", err);
			print_usage(program_name);
			1
		},

		ParseError::UnknownArgument(option, value) => {
			println!("ERROR:  Unknown argument \"{}\" for option \"{}\"", value, option);
			print_usage(program_name);
			1
		},
	};

	std::process::exit(ret_code);
}

#[allow(unused_variables)]
fn on_machine_err(err: MachineError, program_name: &str, ani_opts: &AniOptions) {
	let ret_code = match err {
		MachineError::Io(err) => {
			println!("ERROR:  {}", err);
			1
		},

		MachineError::InvalidArgs => {
			println!("ERROR: {} PLATFORM {}", program_name, ani_opts.platform.unwrap().get_usage());
			1
		},
	};

	std::process::exit(ret_code);
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let program_name = args[0].clone();

	let mut opts = getopts::Options::new();

	match parse_options(&args, &mut opts) {
		Ok((remainder_opts, ani_opts)) => {
			let machine = match ani_opts.platform.unwrap().initialize_machine(&remainder_opts) {
				Ok(machine) => machine,
				Err(err)    => {
					on_machine_err(err, &program_name, &ani_opts);
					return;
				},
			};

			match machine.run() {
				Ok(()) => {
					return;
				},

				Err(err) => {
					on_machine_err(err, &program_name, &ani_opts);
					return;
				},
			}
		},
		Err(err) => {
			on_parse_err(err, &program_name);
		},
	}
}

