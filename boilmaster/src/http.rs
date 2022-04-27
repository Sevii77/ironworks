use std::{fs, io::Read};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use axum_macros::debug_handler;
use ironworks::{
	ffxiv,
	sqpack::{File, SqPack},
};
use tokio::sync::{
	mpsc::{self, Sender},
	oneshot,
};
use tower_http::trace::TraceLayer;

#[derive(thiserror::Error, Debug)]
enum Error {
	#[error("Internal server error.")]
	Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			Self::Other(ref error) => tracing::error!("{error:?}"),
		}

		(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
	}
}

type Result<T, E = Error> = std::result::Result<T, E>;

trait Anyhow<T> {
	fn anyhow(self) -> std::result::Result<T, anyhow::Error>;
}

impl<T, E> Anyhow<T> for std::result::Result<T, E>
where
	E: std::error::Error + Send + Sync + 'static,
{
	fn anyhow(self) -> Result<T, anyhow::Error> {
		self.map_err(anyhow::Error::new)
	}
}

// todo this shouldn't be in http
#[derive(Debug)]
enum IronworksRequest {
	SheetList {
		responder: oneshot::Sender<Result<File<fs::File>, ironworks::Error>>,
	},
}

pub fn router() -> Router {
	// IW isn't async, nor send/sync. Boot up a channel so we can serve requests from a single location.
	// TODO: this seems sane to me but idk maybe iw should be async? idk.
	let (tx, mut rx) = mpsc::channel::<IronworksRequest>(32);

	tokio::spawn(async move {
		// TODO: this should be a configurable path
		let sqpack = SqPack::new(ffxiv::FsResource::search().unwrap());

		while let Some(request) = rx.recv().await {
			use IronworksRequest::*;
			match request {
				SheetList { responder } => {
					// TODO probably need something in iw::excel for listing sheet names publicly
					let file = sqpack.file("exd/root.exl");
					responder.send(file).ok();
				}
			}
		}
	});

	Router::new()
		.route("/sheets", get(sheets))
		.layer(Extension(tx))
		.layer(TraceLayer::new_for_http())
}

#[debug_handler]
async fn sheets(
	Extension(tx): Extension<Sender<IronworksRequest>>,
) -> anyhow::Result<String, Error> {
	let (res_tx, res_rx) = oneshot::channel();
	tx.send(IronworksRequest::SheetList { responder: res_tx })
		.await
		.anyhow()?;

	let mut response = res_rx.await.anyhow()?.anyhow()?;

	let mut string = String::new();
	response.read_to_string(&mut string).anyhow()?;

	Ok(string)
}