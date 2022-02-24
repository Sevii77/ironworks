use glob::glob;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

// TODO: this should probably be in own file
#[derive(Error, Debug)]
pub enum SqPackError {
	#[error("invalid sqpack path \"{0}\"")]
	InvalidPath(String),

	#[error("unknown repository \"{repository}\" in sqpack path \"{path}\"")]
	UnknownRepository { path: String, repository: String },

	#[error("unknown category \"{category}\" in sqpack path \"{path}\"")]
	UnknownCategory { path: String, category: String },
}

#[derive(Debug)]
pub struct SqPack {
	pub repositories: HashMap<String, PathBuf>,
	pub categories: HashMap<String, u8>,

	pub default_repository: String,
}

impl SqPack {
	pub fn temp_test(&self, sqpack_path: &str) -> Result<(), SqPackError> {
		let path = self.parse_path(sqpack_path)?;

		let repository_path = self.repositories.get(&path.repository).ok_or_else(|| {
			SqPackError::UnknownRepository {
				path: path.path.clone(),
				repository: path.repository,
			}
		})?;

		let category_id =
			self.categories
				.get(&path.category)
				.ok_or_else(|| SqPackError::UnknownCategory {
					path: path.path.clone(),
					category: path.category,
				})?;

		println!("repo: {:?}, cat: {}", repository_path, category_id);

		// TODO: Should probably do both index and index2 seperately, and maybe at the same time?
		// TODO: i mean... TODO: index2 lmao
		// TODO: chunks, ex (does ex matter, really? in a repo?), platform?
		// TODO: also this fn is basically a "get_file", this should be done lazily and cached
		let mut index_path = PathBuf::new();
		index_path.push(repository_path);
		index_path.push(format!("{:02x}????.*.index", category_id));

		let indexes = glob(&index_path.to_string_lossy())
			.unwrap()
			.map(|path| path.unwrap())
			.collect::<Vec<PathBuf>>();

		// TODO this is dirty, do stuff better
		if indexes.len() != 1 {
			panic!(
				"too many results in index lookup, fix this shit (chunks?) {:?}",
				indexes
			)
		}

		let index = &indexes[0];

		// TODO: streams? is it even worth it on indexes?
		// TODO: error handling lmao
		let index_bytes = std::fs::read(index).unwrap();

		println!(
			"{:#?}",
			(&index_bytes[0..8])
				.iter()
				.map(|i| i.to_owned() as char)
				.collect::<Vec<char>>()
		);

		return Ok(());
	}

	fn parse_path(&self, sqpack_path: &str) -> Result<SqPackPath, SqPackError> {
		// TODO: Look into itertools or something?
		let lower = sqpack_path.to_lowercase();
		let split = lower.splitn(3, '/').take(2).collect::<Vec<&str>>();
		let (category, mut repository) = match split[..] {
			[category, repository] => (category, repository),
			_ => return Err(SqPackError::InvalidPath(sqpack_path.to_string())),
		};

		if !self.repositories.contains_key(repository) {
			repository = &self.default_repository
		}

		return Ok(SqPackPath {
			category: String::from(category),
			repository: String::from(repository),
			path: lower,
		});
	}
}

// TODO: probs should call this path and namespace on consume
// TODO: I mean realistically this can just be an internal tuple?
#[derive(Debug)]
pub struct SqPackPath {
	path: String,
	category: String,
	repository: String,
}
