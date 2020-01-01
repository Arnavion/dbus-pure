#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::unneeded_field_pattern,
)]

// Connects to the session bus and subscribes to screen lock / unlock events.
// When the screen is locked, it enumerates all media players that implement MPRIS, and pauses them if they were playing.
// When the screen is unlocked, it unpauses all the players it had paused.

fn main() -> Result<(), Error> {
	let connection =
		dbus_pure::conn::Connection::new(
			dbus_pure::conn::BusPath::Session,
			dbus_pure::conn::SaslAuthType::Uid,
		)?;
	let mut client = dbus_pure::client::Client::new(connection)?;

	// Add a match for all screen lock and unlock events. These events manifest as the `org.freedesktop.ScreenSaver.ActiveChanged` signal
	// from the `/org/freedesktop/ScreenSaver` object.
	//
	// Adding a match is done by calling the `org.freedesktop.DBus.AddMatch` method on the `/org/freedesktop/DBus` object
	// at the destination `org.freedesktop.DBus`. The method takes a single string parameter for the match rule.
	client.method_call(
		"org.freedesktop.DBus",
		dbus_pure::types::ObjectPath("/org/freedesktop/DBus".into()),
		"org.freedesktop.DBus",
		"AddMatch",
		Some(&dbus_pure::types::Variant::String(
			"type='signal',path='/org/freedesktop/ScreenSaver',interface='org.freedesktop.ScreenSaver',member='ActiveChanged'".into()
		)),
	)?;

	let mut players_to_resume: std::collections::BTreeSet<_> = Default::default();

	loop {
		let (header, body) = client.recv()?;
		match header.r#type {
			dbus_pure::types::MessageType::Signal { interface, member, path: _ }
				if interface == "org.freedesktop.ScreenSaver" && member == "ActiveChanged" => (),
			_ => continue,
		}

		let locked =
			body
			.ok_or(None)
			.and_then(|body| body.into_bool().map_err(Some))
			.map_err(|body| format!("ActiveChanged signal failed with {:#?}", body))?;

		println!("Screen is {}", if locked { "locked" } else { "unlocked" });

		if locked {
			// List all names by calling the `org.freedesktop.DBus.ListNames` method
			// on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
			let names =
				client.method_call(
					"org.freedesktop.DBus",
					dbus_pure::types::ObjectPath("/org/freedesktop/DBus".into()),
					"org.freedesktop.DBus",
					"ListNames",
					None,
				)?
				.ok_or(None)
				.and_then(|body| body.into_array_string().map_err(Some))
				.map_err(|body| format!("ListNames response failed with {:#?}", body))?;

			// MPRIS media players have names that start with "org.mpris.MediaPlayer2."
			let media_player_names =
				names.into_owned().into_iter()
				.filter(|object_name| object_name.starts_with("org.mpris.MediaPlayer2."));

			for media_player_name in media_player_names {
				// Get the playback status of the media player by gettings its `PlaybackStatus` property.
				//
				// The property is exposed by the object at path `/org/mpris/MediaPlayer2`
				// on the `org.mpris.MediaPlayer2.Player` interface.
				//
				// Properties in general are accessed by calling the `org.freedesktop.DBus.Properties.Get` method
				// with two parameters - the interface name and the property name.
				let playback_status =
					client.method_call(
						&media_player_name,
						dbus_pure::types::ObjectPath("/org/mpris/MediaPlayer2".into()),
						"org.freedesktop.DBus.Properties",
						"Get",
						Some(&dbus_pure::types::Variant::Tuple {
							elements: (&[
								dbus_pure::types::Variant::String("org.mpris.MediaPlayer2.Player".into()),
								dbus_pure::types::Variant::String("PlaybackStatus".into()),
							][..]).into(),
						}),
					)?
					.ok_or(None)
					.and_then(|body| body.into_variant().map_err(Some))
					.and_then(|body| body.into_owned().into_string().map_err(Some))
					.map_err(|body| format!("GetPlaybackStatus response failed with {:#?}", body))?;

				if playback_status == "Playing" {
					println!("Pausing {} ...", media_player_name);

					// Pause the player by invoking its `org.mpris.MediaPlayer2.Player.Pause` method.
					client.method_call(
						&media_player_name,
						dbus_pure::types::ObjectPath("/org/mpris/MediaPlayer2".into()),
						"org.mpris.MediaPlayer2.Player",
						"Pause",
						None,
					)?;

					println!("{} is paused", media_player_name);

					players_to_resume.insert(media_player_name.into_owned());
				}
			}
		}
		else {
			for media_player_name in std::mem::take(&mut players_to_resume) {
				println!("Unpausing {} ...", media_player_name);

				// Unpause the player by invoking its `org.mpris.MediaPlayer2.Player.Play` method.
				// Swallow any errors in case the player refuses to play or no longer exists.
				let result = client.method_call(
					&media_player_name,
					dbus_pure::types::ObjectPath("/org/mpris/MediaPlayer2".into()),
					"org.mpris.MediaPlayer2.Player",
					"Play",
					None,
				);
				if result.is_ok() {
					println!("{} is unpaused", media_player_name);
				}
			}
		}
	}
}

struct Error(Box<dyn std::error::Error>);

impl<E> From<E> for Error where E: Into<Box<dyn std::error::Error>> {
	fn from(err: E) -> Self {
		Error(err.into())
	}
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;

		let mut source = self.0.source();
		while let Some(err) = source {
			writeln!(f, "caused by: {}", err)?;
			source = err.source();
		}

		Ok(())
	}
}
