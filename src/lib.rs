//! *Read and write user-specific application data*
//!
//! This crate allows Rust developers to store and retrieve user-local preferences and other
//! application data in a flexible and platform-appropriate way.
//!
//! Though it was originally inspired by Java's convenient
//! [Preferences API](https://docs.oracle.com/javase/8/docs/api/java/util/prefs/Preferences.html),
//! this crate is more flexible; any type that implements
//! [`rustc-serialize`][rustc-serialize-api]'s `Encodable` and `Decodable`
//! traits can be stored and retrieved as user data! Thankfully, implementing those traits is
//! trivial; just use `#[derive(RustcEncodable, RustcDecodable)`.
//!
//! # Usage
//! For convenience, the type [`PreferencesMap<T>`](type.PreferencesMap.html) is provided. (It's
//! actually just a [`std::collections::HashMap<String, T>`][hashmap-api], where `T` defaults to
//! `String`). This mirrors the Java API, which models user data as an opaque key-value store. As
//! long as the map is instantiated over a type `T` which is serializable and deserializable,
//! [`PreferencesTrait`](trait.PreferencesTrait.html) will be implemented for your map instance.
//! This will allow you to seamlessly save and load user data with the `save(..)` and `load(..)`
//! methods on `PreferencesTrait`.
//!
//! # Roadmap
//! This crate aims to provide a convenient API for both stable and nightly Rust, which is why
//! it currently uses [`rustc-serialize`][rustc-serialize-api] instead of the more recent
//! [`serde`][serde-api] library. In the distant future, when compiler plugins are stabilized
//! and `serde` is available in stable Rust, this library will migrate to `serde`. This will be
//! a breaking change (and will update the semantic version number accordingly so that your
//! builds don't break). At that point, updating should be dead simple; you'll just have to
//! replace `#[derive(RustcEncodable, RustcDecodable)` with `#[derive(Serialize, Deserialize)`,
//! and only if you store custom data types in your user data.
//!
//! # Basic example
//! ```
//! extern crate preferences;
//! use preferences::{PreferencesMap, PreferencesTrait};
//!
//! fn main() {
//!
//!     // Create a new preferences key-value map
//!     // (Under the hood: HashMap<String, String>)
//!     let mut faves: PreferencesMap<String> = PreferencesMap::new();
//!
//!     // Edit the preferences (std::collections::HashMap)
//!     faves.insert("color".into(), "blue".into());
//!     faves.insert("programming language".into(), "Rust".into());
//!
//!     // Store the user's preferences
//!     let prefs_key = "preferences-rs/examples/faves";
//!     faves.save(prefs_key);
//!
//!     // ... Then do some stuff ...
//!
//!     // Retrieve the user's preferences
//!     let mut loaded_faves = PreferencesMap::new();
//!     let load_result = loaded_faves.load(prefs_key);
//!     assert!(load_result.is_ok());
//!     assert_eq!(loaded_faves, faves);
//!
//! }
//! ```
//!
//! # Using custom data types
//! ```
//! extern crate rustc_serialize;
//! extern crate preferences;
//! use preferences::{PreferencesMap, PreferencesTrait};
//!
//! #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
//! struct PlayerData {
//!     level: u32,
//!     health: f32,
//! }
//!
//! fn main() {
//!
//!     let player = PlayerData{level: 2, health: 0.75};
//!
//!     let prefs_key = "preferences-rs/examples/player";
//!     player.save(prefs_key);
//!
//!     let mut loaded_player = PlayerData{level: 0, health: 0.0};
//!     let load_result = loaded_player.load(prefs_key);
//!     assert!(load_result.is_ok());
//!     assert_eq!(loaded_player, player);
//!
//! }
//! ```
//!
//! # Using custom data types with `PreferencesMap`
//! ```
//! extern crate rustc_serialize;
//! extern crate preferences;
//! use preferences::{PreferencesMap, PreferencesTrait};
//!
//! #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
//! struct Point(f32, f32);
//!
//! fn main() {
//!
//!     let mut places = PreferencesMap::new();
//!     places.insert("treasure".into(), Point(1.0, 1.0));
//!     places.insert("home".into(), Point(-1.0, 6.6));
//!
//!     let prefs_key = "preferences-rs/examples/places";
//!     places.save(prefs_key);
//!
//!     let mut loaded_places = PreferencesMap::new();
//!     let load_result = loaded_places.load(prefs_key);
//!     assert!(load_result.is_ok());
//!     assert_eq!(loaded_places, places);
//!
//! }
//! ```
//!
//! # Using custom data types with serializable containers
//! ```
//! extern crate rustc_serialize;
//! extern crate preferences;
//! use preferences::{PreferencesMap, PreferencesTrait};
//!
//! #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
//! struct Point(usize, usize);
//!
//! fn main() {
//!
//!     let square = vec![
//!         Point(0,0),
//!         Point(1,0),
//!         Point(1,1),
//!         Point(0,1),
//!     ];
//!
//!     let prefs_key = "preferences-rs/examples/square";
//!     square.save(prefs_key);
//!
//!     let mut loaded_square: Vec<Point> = Vec::new();
//!     let load_result = loaded_square.load(prefs_key);
//!     assert!(load_result.is_ok());
//!     assert_eq!(loaded_square, square);
//!
//! }
//! ```
//!
//! # Under the hood
//! Data is written to flat files under the active user's home directory in a location specific to
//! the operating system.
//!
//! * Mac OS X: `~/Library/Preferences`
//! * Other Unix/Linux: `~/.config`
//! * Windows: `%USERPROFILE%\AppData\Roaming` (a.k.a. `%APPDATA%`)
//!
//! The data is stored in JSON format. This has several advantages:
//!
//! * Human-readable and self-describing
//! * More compact than e.g. XML
//! * Better adoption rates and language compatibility than e.g. TOML
//! * Not reliant on a consistent memory layout like e.g. binary
//!
//! You could, of course, implement `PreferencesTrait` yourself and store your user data in
//! whatever location and format that you wanted. But that would defeat the purpose of this
//! library. &#128522;
//!
//! [hashmap-api]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
//! [rustc-serialize-api]: https://crates.io/crates/rustc-serialize
//! [serde-api]: https://crates.io/crates/serde

#![warn(missing_docs)]

extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::json::{self, EncoderError, DecoderError};
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;

type IoError = std::io::Error;

#[cfg(target_os="macos")]
static PREFS_DIR_PATH: &'static str = "Library/Preferences";
#[cfg(all(unix, not(target_os="macos")))]
static PREFS_DIR_PATH: &'static str = ".config";
#[cfg(windows)]
static PREFS_DIR_PATH: &'static str = "AppData/Roaming";

/// Generic key-value store for user data.
///
/// This is actually a wrapper type around [`std::collections::HashMap<String, T>`][hashmap-api]
/// (with `T` defaulting to `String`), so use the `HashMap` API methods to access and change user
/// data in memory.
///
/// To save or load user data, use the methods defined for
/// [`PreferencesTrait`](trait.PreferencesTrait.html), which will be automatically implemented for
/// `PreferencesMap<T>` as long as `T` is serializable. (See the
/// [module documentation](index.html) for examples and more details.)
///
/// [hashmap-api]: https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html
pub type PreferencesMap<T = String> = HashMap<String, T>;

/// Error type representing the errors that can occur when saving or loading user data.
#[derive(Debug)]
pub enum PreferencesError {
    /// An error occurred during JSON (serialization.
    Serialize(EncoderError),
    /// An error occurred during JSON deserialization.
    Deserialize(DecoderError),
    /// An error occurred during file I/O.
    Io(std::io::Error),
}

impl From<EncoderError> for PreferencesError {
    fn from(e: EncoderError) -> Self {
        PreferencesError::Serialize(e)
    }
}

impl From<DecoderError> for PreferencesError {
    fn from(e: DecoderError) -> Self {
        PreferencesError::Deserialize(e)
    }
}

impl From<FromUtf8Error> for PreferencesError {
    fn from(_: FromUtf8Error) -> Self {
        let kind = ErrorKind::InvalidData;
        let msg = "Preferences file contained invalid UTF-8";
        let err = IoError::new(kind, msg);
        PreferencesError::Io(err)
    }
}

impl From<std::io::Error> for PreferencesError {
    fn from(e: std::io::Error) -> Self {
        PreferencesError::Io(e)
    }
}

/// Trait for types that can be saved & loaded as user data.
///
/// This type is automatically implemented for any type `T` which implements both `Encodable` and
/// `Decodable` (from `rustc-serialize`). However, you are encouraged to use the provided type,
/// [`PreferencesMap`](type.PreferencesMap.html).
///
/// The `path` parameter of `save(..)` and `load(..)` should be a valid, relative file path. It is
/// *highly* recommended that you use the format
/// `[company or author]/[application name]/[data description]`. For example, a game might use
/// the following paths for player options and save data, respectively:
///
/// * `fun-games-inc/awesome-game-2/options`
/// * `fun-games-inc/awesome-game-2/saves`
pub trait PreferencesTrait {
    /// Saves the current state of this object. Implementation is platform-dependent, but the data
    /// will be local to the active user. For more details, see
    /// [the module documentation](index.html).
    ///
    /// # Failures
    /// If a serialization or file I/O error occurs (e.g. permission denied), or if the provided
    /// `path` argument is invalid.
    fn save<S>(&self, path: S) -> Result<(), PreferencesError> where S: AsRef<str>;
    /// Loads this object's state from previously saved user data with the same `path`. This is
    /// an instance method which completely overwrites the object's state with the serialized
    /// data. Thus, it is recommended that you call this method immediately after instantiating
    /// the preferences object.
    ///
    /// # Failures
    /// If a deserialization or file I/O error occurs (e.g. permission denied), if the provided
    /// `path` argument is invalid, or if no user data exists at that `path`.
    fn load<S>(&mut self, path: S) -> Result<(), PreferencesError> where S: AsRef<str>;
}

impl<T> PreferencesTrait for T
    where T: Encodable + Decodable
{
    fn save<S>(&self, path: S) -> Result<(), PreferencesError>
        where S: AsRef<str>
    {
        let path = try!(path_buf_from_name(path.as_ref()));
        path.parent().map(create_dir_all);
        let mut file = try!(File::create(path));
        let encoded = try!(json::encode(self));
        try!(file.write_all(encoded.as_bytes()));
        try!(file.flush());
        Ok(())
    }
    fn load<S>(&mut self, path: S) -> Result<(), PreferencesError>
        where S: AsRef<str>
    {
        let path = try!(path_buf_from_name(path.as_ref()));
        let mut file = try!(File::open(path));
        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));
        let encoded = try!(String::from_utf8(bytes));
        let new_self = try!(json::decode(&encoded));
        *self = new_self;
        Ok(())
    }
}

fn get_prefs_base_path() -> Option<PathBuf> {
    std::env::home_dir().map(|mut dir| {
        dir.push(PREFS_DIR_PATH);
        dir
    })
}

fn path_buf_from_name(name: &str) -> Result<PathBuf, IoError> {

    let msg_not_found = "Could not find home directory for user data storage";
    let err_not_found = IoError::new(ErrorKind::NotFound, msg_not_found);

    let msg_bad_name = "Invalid preferences name: ".to_owned() + name;
    let err_bad_name = Result::Err(IoError::new(ErrorKind::Other, msg_bad_name));

    if name.starts_with("../") || name.ends_with("/..") || name.contains("/../") {
        return err_bad_name;
    }
    let mut base_path = try!(get_prefs_base_path().ok_or(err_not_found));
    let name_path = Path::new(name);
    if !name_path.is_relative() {
        return err_bad_name;
    }
    base_path.push(name_path);
    Result::Ok(base_path)
}

#[cfg(test)]
mod tests {
    use {PreferencesTrait, PreferencesMap};
    static TEST_PREFIX: &'static str = "rust_user_prefs_test";
    fn gen_test_name(name: &str) -> String {
        TEST_PREFIX.to_owned() + "/" + name
    }
    fn gen_sample_prefs() -> PreferencesMap<String> {
        let mut prefs = PreferencesMap::new();
        prefs.insert("foo".into(), "bar".into());
        prefs.insert("age".into(), "23".into());
        prefs.insert("PI".into(), "3.14".into());
        prefs.insert("offset".into(), "-9".into());
        prefs
    }
    #[test]
    fn test_save_load() {
        let name = gen_test_name("/save_load");
        let sample = gen_sample_prefs();
        let save_result = sample.save(&name);
        println!("Save result: {:?}", save_result);
        assert!(save_result.is_ok());
        let mut loaded_map = PreferencesMap::new();
        let load_result = loaded_map.load(&name);
        println!("Load result: {:?}", load_result);
        assert!(load_result.is_ok());
        assert_eq!(loaded_map, sample);
    }
}
