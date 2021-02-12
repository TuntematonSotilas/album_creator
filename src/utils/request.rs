use base64::encode;
use seed::{self, prelude::*};

use crate::{
	utils::{
		deserializer::{deser_album_det, deser_picture},
		vars::{LOGIN, PWD, API_URI}
	},
	models::album::Album,
};

pub fn get_auth() -> String {
	let cred = format!("{0}:{1}", LOGIN, PWD);
	let encoded = encode(cred);
	return format!("Basic {0}", encoded);
}

pub async fn get_album(id: String) -> Option<Album> {
	let uri = format!("{0}get-album-detail?id={1}", API_URI, id);
	let request = Request::new(uri)
		.method(Method::Get)
		.header(Header::authorization(get_auth()));
	let result = fetch(request).await;
	deser_album_det(result).await
}

pub async fn get_picture(id: String) -> Option<String> {
	let uri = format!("{0}get-picture?id={1}", API_URI, id);
	let request = Request::new(uri)
		.method(Method::Get)
		.header(Header::authorization(get_auth()));
	let result = fetch(request).await;
	deser_picture(result).await
}