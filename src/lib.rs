#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::let_and_return,
	clippy::let_unit_value,
	clippy::missing_errors_doc,
	clippy::must_use_candidate,
	clippy::shadow_unrelated,
	clippy::unneeded_field_pattern,
)]

//! This is a pure Rust implementation of a D-Bus client.
//!
//! Create a client with [`Client::new`]
//!
//!
//! # Example
//!
//! ## Connect to the session bus and list all names, with the D-Bus interfaces defined using the `dbus-pure-macros` crate's macros
//!
//! ```rust
//! /// The `org.freedesktop.DBus` interface with the `ListNames` method.
//! #[dbus_pure_macros::interface("org.freedesktop.DBus")]
//! trait OrgFreeDesktopDbusInterface {
//!     #[name = "ListNames"]
//!     fn list_names() -> Vec<String>;
//! }
//!
//! /// The `/org/freedesktop/DBus` object.
//! #[dbus_pure_macros::object(OrgFreeDesktopDbusInterface)]
//! struct OrgFreeDesktopDbusObject;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #
//! let connection =
//!     dbus_pure::Connection::new(
//!         dbus_pure::BusPath::Session,
//!         dbus_pure::SaslAuthType::Uid,
//!     )?;
//! let mut client = dbus_pure::Client::new(connection)?;
//!
//! // List all names by calling the `org.freedesktop.DBus.ListNames` method
//! // on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
//! let names = {
//!     let obj = OrgFreeDesktopDbusObject {
//!         name: "org.freedesktop.DBus".into(),
//!         path: dbus_pure::proto::ObjectPath("/org/freedesktop/DBus".into()),
//!     };
//!     let names = obj.list_names(&mut client)?;
//!     names
//! };
//!
//! for name in names {
//!     println!("{}", name);
//! }
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ## Connect to the session bus and list all names manually
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #
//! let connection =
//!     dbus_pure::Connection::new(
//!         dbus_pure::BusPath::Session,
//!         dbus_pure::SaslAuthType::Uid,
//!     )?;
//! let mut client = dbus_pure::Client::new(connection)?;
//!
//! // List all names by calling the `org.freedesktop.DBus.ListNames` method
//! // on the `/org/freedesktop/DBus` object at the destination `org.freedesktop.DBus`.
//! let names = {
//!     let body =
//!         client.method_call(
//!             "org.freedesktop.DBus",
//!             dbus_pure::proto::ObjectPath("/org/freedesktop/DBus".into()),
//!             "org.freedesktop.DBus",
//!             "ListNames",
//!             None,
//!         )?
//!         .ok_or("ListNames response does not have a body")?;
//!     let body: Vec<String> = serde::Deserialize::deserialize(body)?;
//!     body
//! };
//!
//! for name in names {
//!     println!("{}", name);
//! }
//! #
//! # Ok(())
//! # }
//! ```

pub use dbus_pure_proto as proto;

mod client;
pub use client::{
	Client,
	CreateClientError,
	MethodCallError,
};

mod conn;
pub use conn::{
	BusPath,
	ConnectError,
	Connection,
	RecvError,
	SaslAuthType,
	SendError,
};
