use std::env;
use std::fs;
use tokio;
use futures::StreamExt;
use telegram_bot::*;

use crate::Sotpal;

#[tokio::main]
pub async fn run() {
	let token = fs::read_to_string("secrets/token")
        .expect("Could not read from file");
	let api = Api::new(token);
	
	let mut stream = api.stream();
	while let Some(update) = stream.next().await {
		let update = update.unwrap();
		if let UpdateKind::Message(message) = update.kind {
			if let MessageKind::Text { ref data, .. } = message.kind {
				println!("<{}>: {}", &message.from.first_name, data);

				api.send(message.text_reply(format!(
					"Hi, {}! You just wrote '{}'",
					&message.from.first_name, data
				)))
				.await;
			}
		}
	}
	
	let mut game = Sotpal::new();
}