#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::let_and_return,
	clippy::let_unit_value,
	clippy::missing_errors_doc,
	clippy::module_name_repetitions,
	clippy::must_use_candidate,
	clippy::shadow_unrelated,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unneeded_field_pattern,
	clippy::unknown_clippy_lints,
	clippy::use_self,
)]

//! This is a pure Rust implementation of a D-Bus client.
//!
//! Create a client with [`client::Client::new`]
//!
//!
//! # Example
//!
//! ## Connect to the session bus and list all names
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #
//! let connection =
//!     dbus_pure::conn::Connection::new(
//!         dbus_pure::conn::BusPath::Session,
//!         dbus_pure::conn::SaslAuthType::Uid,
//!     )?;
//! let mut client = dbus_pure::client::Client::new(connection)?;
//!
//! // List all names by calling the `org.freedesktop.DBus.ListNames` method
//! // on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
//! let names =
//!     client.method_call(
//!         "org.freedesktop.DBus",
//!         dbus_pure::types::ObjectPath("/org/freedesktop/DBus".into()),
//!         "org.freedesktop.DBus",
//!         "ListNames",
//!         None,
//!     )?
//!     .ok_or(None)
//!     .and_then(|body| body.into_array_string().map_err(Some))
//!     .map_err(|body| format!("ListNames response failed with {:#?}", body))?;
//!
//! for name in names.into_iter() {
//!     println!("{}", name);
//! }
//! #
//! # Ok(())
//! # }
//! ```

pub mod client;

pub mod conn;

pub(crate) mod de;

pub(crate) mod ser;

pub mod std2;

pub mod types;
