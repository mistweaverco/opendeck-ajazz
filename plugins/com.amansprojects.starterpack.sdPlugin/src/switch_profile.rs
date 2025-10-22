use openaction::*;

// Non-spec OpenDeck-specific protocols are used in this file.

#[derive(serde::Serialize)]
struct SwitchProfileEvent {
	event: &'static str,
	device: String,
	profile: String,
}

pub async fn key_up(event: KeyEvent, outbound: &mut OutboundEventManager) -> EventHandlerResult {
	outbound
		.send_event(SwitchProfileEvent {
			event: "switchProfile",
			device: event
				.payload
				.settings
				.as_object()
				.and_then(|x| x.get("device"))
				.and_then(|x| x.as_str())
				.unwrap_or(&event.device)
				.to_owned(),
			profile: event
				.payload
				.settings
				.as_object()
				.and_then(|x| x.get("profile"))
				.and_then(|x| x.as_str())
				.unwrap_or("Default")
				.to_owned(),
		})
		.await?;

	Ok(())
}
