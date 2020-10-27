use base64::encode;

use crate::utils::vars::{LOGIN, PWD};

pub fn get_auth() -> String {
	let cred = format!("{0}:{1}", LOGIN, PWD);
	let encoded = encode(cred);
	return format!("Basic {0}", encoded);
}