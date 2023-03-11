#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::let_and_return,
	clippy::let_unit_value,
)]

// Connects to the session bus and subscribes to screen lock / unlock events.
// When the screen is locked, it enumerates all media players that implement MPRIS, and pauses them if they were playing.
// When the screen is unlocked, it unpauses all the players it had paused.

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
			return Err(format!(r#"invalid value of FORCE_WRITE_ENDIANNESS env var {s:?}, expected "big" or "little""#).into());
		}
	}

	let mut client = dbus_pure::Client::new(connection)?;

	// Add a match for all screen lock and unlock events. These events manifest as the `org.freedesktop.ScreenSaver.ActiveChanged` signal
	// from the `/org/freedesktop/ScreenSaver` object.
	//
	// Adding a match is done by calling the `org.freedesktop.DBus.AddMatch` method on the `/org/freedesktop/DBus` object
	// at the destination `org.freedesktop.DBus`. The method takes a single string parameter for the match rule.
	{
		let obj = OrgFreeDesktopDbusObject {
			name: "org.freedesktop.DBus".into(),
			path: dbus_pure::proto::ObjectPath("/org/freedesktop/DBus".into()),
		};
		let () =
			obj.add_match(
				&mut client,
				"type='signal',path='/org/freedesktop/ScreenSaver',interface='org.freedesktop.ScreenSaver',member='ActiveChanged'",
			)?;
	}

	let mut players_to_resume: std::collections::BTreeSet<_> = Default::default();

	loop {
		let locked = {
			let (header, body) = client.recv()?;
			match header.r#type {
				dbus_pure::proto::MessageType::Signal { interface, member, path: _ }
					if interface == "org.freedesktop.ScreenSaver" && member == "ActiveChanged" => (),
				_ => continue,
			}

			let body = body.ok_or("ActiveChanged signal does not have a body")?;
			let body: bool = serde::Deserialize::deserialize(body)?;
			body
		};

		println!("Screen is {}", if locked { "locked" } else { "unlocked" });

		if locked {
			// List all names by calling the `org.freedesktop.DBus.ListNames` method
			// on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
			let names = {
				let obj = OrgFreeDesktopDbusObject {
					name: "org.freedesktop.DBus".into(),
					path: dbus_pure::proto::ObjectPath("/org/freedesktop/DBus".into()),
				};
				let names = obj.list_names(&mut client)?;
				names
			};

			// MPRIS media players have names that start with "org.mpris.MediaPlayer2."
			let media_player_names =
				names.into_iter()
				.filter(|object_name| object_name.starts_with("org.mpris.MediaPlayer2."));

			for media_player_name in media_player_names {
				let obj = OrgMprisMediaPlayer2Object {
					name: (&*media_player_name).into(),
					path: dbus_pure::proto::ObjectPath("/org/mpris/MediaPlayer2".into()),
				};

				// Get the playback status of the media player by gettings its `PlaybackStatus` property.
				//
				// The property is exposed by the object at path `/org/mpris/MediaPlayer2`
				// on the `org.mpris.MediaPlayer2.Player` interface.
				//
				// Properties in general are accessed by calling the `org.freedesktop.DBus.Properties.Get` method
				// with two parameters - the interface name and the property name.
				let playback_status = {
					let playback_status = obj.get(&mut client, "org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;
					let playback_status: String = serde::Deserialize::deserialize(playback_status)?;
					playback_status
				};

				if playback_status == "Playing" {
					println!("Pausing {media_player_name} ...");

					// Pause the player by invoking its `org.mpris.MediaPlayer2.Player.Pause` method.
					let () = obj.pause(&mut client)?;

					println!("{media_player_name} is paused");

					players_to_resume.insert(media_player_name);
				}
			}
		}
		else {
			for media_player_name in std::mem::take(&mut players_to_resume) {
				let obj = OrgMprisMediaPlayer2Object {
					name: (&*media_player_name).into(),
					path: dbus_pure::proto::ObjectPath("/org/mpris/MediaPlayer2".into()),
				};

				println!("Unpausing {media_player_name} ...");

				// Unpause the player by invoking its `org.mpris.MediaPlayer2.Player.Play` method.
				// Swallow any errors in case the player refuses to play or no longer exists.
				let result = obj.play(&mut client);
				if result.is_ok() {
					println!("{media_player_name} is unpaused");
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
			writeln!(f, "caused by: {err}")?;
			source = err.source();
		}

		Ok(())
	}
}

#[dbus_pure_macros::interface("org.freedesktop.DBus")]
trait OrgFreeDesktopDbusInterface {
	#[name = "AddMatch"]
	fn add_match(rule: &str);

	#[name = "ListNames"]
	fn list_names() -> Vec<String>;
}

#[dbus_pure_macros::object(OrgFreeDesktopDbusInterface)]
struct OrgFreeDesktopDbusObject;

#[dbus_pure_macros::interface("org.freedesktop.DBus.Properties")]
trait OrgFreeDesktopDbusPropertiesInterface {
	#[name = "Get"]
	fn get(interface_name: &str, property_name: &str) -> dbus_pure::proto::Variant<'static>;
}

#[dbus_pure_macros::interface("org.mpris.MediaPlayer2.Player")]
trait OrgMprisMediaPlayer2Player {
	#[name = "Pause"]
	fn pause();

	#[name = "Play"]
	fn play();
}

#[dbus_pure_macros::object(
	OrgFreeDesktopDbusPropertiesInterface,
	OrgMprisMediaPlayer2Player,
)]
struct OrgMprisMediaPlayer2Object;
