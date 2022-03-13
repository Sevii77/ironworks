use std::collections::HashSet;

use crate::error::Error;

pub struct ExcelList {
	sheets: HashSet<String>,
}

impl ExcelList {
	// TODO: should this move the bytes?
	pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
		// Binary format is actually just text.
		let mut lines = std::str::from_utf8(bytes)
			.map_err(|error| {
				Error::InvalidResource(format!("Invalid utf8 sequence in ExcelList: {}", error))
			})?
			.split("\r\n");

		// First line is a magic, make sure we've got one.
		match lines.next().map(|line| &line[0..4]) {
			Some("EXLT") => (),
			_ => {
				return Err(Error::InvalidResource(
					"Missing EXLT magic in ExcelList".into(),
				))
			}
		}

		// Build the set of sheets. We're ignoring the sheet ID, as it's pretty
		// irrelevant for us at this stage.
		let sheets = lines
			.filter_map(|line| {
				let index = line.find(',')?;
				Some(line[0..index].to_string())
			})
			.collect::<HashSet<_>>();

		Ok(Self { sheets })
	}

	pub fn has_sheet(&self, sheet_name: &str) -> bool {
		self.sheets.contains(sheet_name)
	}
}
