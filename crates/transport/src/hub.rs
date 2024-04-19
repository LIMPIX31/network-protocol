use std::iter;
use tracing::info;

use web_sys::RtcConfiguration;

use crate::{Port, Result};

#[derive(Debug)]
pub struct Hub {
	ports: Vec<Port>,
}

impl Hub {
	pub fn new(size: usize, config: &RtcConfiguration) -> Result<Self> {
		let factory = iter::repeat_with(|| Port::new(config));

		let ports = factory.take(size).collect::<Result<_, _>>()?;
		let this = Self { ports };

		info!(size, ?config, "New hub initialized with {size} ports");

		Ok(this)
	}
}
