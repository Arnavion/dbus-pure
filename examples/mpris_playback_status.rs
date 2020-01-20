#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]

// Connects to the session bus, enumerates all media players that implement MPRIS, and prints their playback status.

fn main() -> Result<(), Error> {
	let mut connection =
		dbus_pure::Connection::new(
			dbus_pure::BusPath::Session,
			dbus_pure::SaslAuthType::Uid,
		)?;

	// For testing
	if let Some(s) = std::env::var_os("FORCE_WRITE_ENDIANNESS") {
		if s == "big" {
			connection.set_write_endianness(dbus_pure::proto::Endianness::Big);
		}
		else if s == "little" {
			connection.set_write_endianness(dbus_pure::proto::Endianness::Little);
		}
		else {
			return Err(format!(r#"invalid value of FORCE_WRITE_ENDIANNESS env var {:?}, expected "big" or "little""#, s).into());
		}
	}

	let mut client = dbus_pure::Client::new(connection)?;

	// List all names by calling the `org.freedesktop.DBus.ListNames` method
	// on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
	let names = {
		let body =
			client.method_call(
				"org.freedesktop.DBus",
				dbus_pure::proto::ObjectPath("/org/freedesktop/DBus".into()),
				"org.freedesktop.DBus",
				"ListNames",
				None,
			)?
			.ok_or("ListNames response does not have a body")?;
		let body: Vec<String> = serde::Deserialize::deserialize(body)?;
		body
	};

	// MPRIS media players have names that start with "org.mpris.MediaPlayer2."
	let media_player_names = names.iter().filter(|object_name| object_name.starts_with("org.mpris.MediaPlayer2."));

	for media_player_name in media_player_names {
		println!("Found media player {}", media_player_name);

		// Get the playback status of the media player by gettings its `PlaybackStatus` property.
		//
		// The property is exposed by the object at path `/org/mpris/MediaPlayer2`
		// on the `org.mpris.MediaPlayer2.Player` interface.
		//
		// Properties in general are accessed by calling the `org.freedesktop.DBus.Properties.Get` method
		// with two parameters - the interface name and the property name.
		let playback_status = {
			let body =
				client.method_call(
					media_player_name,
					dbus_pure::proto::ObjectPath("/org/mpris/MediaPlayer2".into()),
					"org.freedesktop.DBus.Properties",
					"Get",
					Some(&dbus_pure::proto::Variant::Tuple {
						elements: (&[
							dbus_pure::proto::Variant::String("org.mpris.MediaPlayer2.Player".into()),
							dbus_pure::proto::Variant::String("PlaybackStatus".into()),
						][..]).into(),
					}),
				)?
				.ok_or("GetPlaybackStatus response does not have a body")?;
			let body: String = serde::Deserialize::deserialize(body)?;
			body
		};

		println!("{} is {}", media_player_name, playback_status);
	}

	Ok(())
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
