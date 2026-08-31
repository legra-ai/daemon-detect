//! Validated domain values used by markers and service identities.
//!
//! Each wrapper enforces the limits of the OS surface the value is
//! handed to at construction time, so detection and label derivation
//! can never emit a malformed account lookup, environment probe, or
//! service label.

use std::fmt;

/// Why a service value was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidServiceValue {
    kind: &'static str,
    reason: &'static str,
    value: String,
}

impl InvalidServiceValue {
    fn new(kind: &'static str, reason: &'static str, value: &str) -> Self {
        Self {
            kind,
            reason,
            value: value.to_owned(),
        }
    }
}

impl fmt::Display for InvalidServiceValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid {}: {} ({:?})",
            self.kind, self.reason, self.value
        )
    }
}

impl std::error::Error for InvalidServiceValue {}

/// A Unix service-account name, as passed to `getpwnam(3)`.
///
/// Limits: 1–32 bytes, ASCII graphic characters only, and none of
/// `:`, `/`, or whitespace (the passwd-database separators). macOS
/// underscore-prefixed accounts (`_mydaemon`) are valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceAccountName(String);

impl ServiceAccountName {
    /// Validate and wrap a service-account name.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidServiceValue`] when the name is empty, longer
    /// than 32 bytes, or contains a non-graphic ASCII byte, `:`, or
    /// `/`.
    pub fn try_new(name: impl Into<String>) -> Result<Self, InvalidServiceValue> {
        const KIND: &str = "service-account name";
        let name = name.into();
        if name.is_empty() {
            return Err(InvalidServiceValue::new(KIND, "must not be empty", &name));
        }
        if name.len() > 32 {
            return Err(InvalidServiceValue::new(
                KIND,
                "longer than 32 bytes",
                &name,
            ));
        }
        if !name.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must be ASCII graphic characters",
                &name,
            ));
        }
        if name.contains([':', '/']) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must not contain ':' or '/'",
                &name,
            ));
        }
        Ok(Self(name))
    }

    /// The validated account name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An environment-variable name, as probed with `std::env::var_os`.
///
/// Limits: 1–128 bytes, ASCII graphic characters only, and no `=`
/// (the environment-block separator).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVarName(String);

impl EnvVarName {
    /// Validate and wrap an environment-variable name.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidServiceValue`] when the name is empty, longer
    /// than 128 bytes, contains a non-graphic ASCII byte, or contains
    /// `=`.
    pub fn try_new(name: impl Into<String>) -> Result<Self, InvalidServiceValue> {
        const KIND: &str = "environment-variable name";
        let name = name.into();
        if name.is_empty() {
            return Err(InvalidServiceValue::new(KIND, "must not be empty", &name));
        }
        if name.len() > 128 {
            return Err(InvalidServiceValue::new(
                KIND,
                "longer than 128 bytes",
                &name,
            ));
        }
        if !name.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must be ASCII graphic characters",
                &name,
            ));
        }
        if name.contains('=') {
            return Err(InvalidServiceValue::new(
                KIND,
                "must not contain '='",
                &name,
            ));
        }
        Ok(Self(name))
    }

    /// The validated variable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An installed binary's file name, used in systemd template units
/// (`{binary}@{instance}.service`) and Windows service names
/// (`{binary}-{instance}`).
///
/// Limits: 1–64 bytes, ASCII graphic characters only, and none of
/// `/`, `\`, or `@` (path and systemd-template separators). A
/// platform suffix such as `.exe` is valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryName(String);

impl BinaryName {
    /// Validate and wrap a binary name.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidServiceValue`] when the name is empty, longer
    /// than 64 bytes, contains a non-graphic ASCII byte, or contains
    /// `/`, `\`, or `@`.
    pub fn try_new(name: impl Into<String>) -> Result<Self, InvalidServiceValue> {
        const KIND: &str = "binary name";
        let name = name.into();
        if name.is_empty() {
            return Err(InvalidServiceValue::new(KIND, "must not be empty", &name));
        }
        if name.len() > 64 {
            return Err(InvalidServiceValue::new(
                KIND,
                "longer than 64 bytes",
                &name,
            ));
        }
        if !name.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must be ASCII graphic characters",
                &name,
            ));
        }
        if name.contains(['/', '\\', '@']) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must not contain '/', '\\', or '@'",
                &name,
            ));
        }
        Ok(Self(name))
    }

    /// The validated binary name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A reverse-DNS launchd label prefix (e.g. `com.example.my-daemon`).
///
/// Limits: 1–128 bytes, one or more non-empty dot-separated segments
/// of ASCII alphanumerics, `-`, or `_`; no leading, trailing, or
/// doubled dots.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LaunchdLabelPrefix(String);

impl LaunchdLabelPrefix {
    /// Validate and wrap a launchd label prefix.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidServiceValue`] when the prefix is empty,
    /// longer than 128 bytes, or not dot-separated segments of ASCII
    /// alphanumerics, `-`, or `_`.
    pub fn try_new(prefix: impl Into<String>) -> Result<Self, InvalidServiceValue> {
        const KIND: &str = "launchd label prefix";
        let prefix = prefix.into();
        if prefix.is_empty() {
            return Err(InvalidServiceValue::new(KIND, "must not be empty", &prefix));
        }
        if prefix.len() > 128 {
            return Err(InvalidServiceValue::new(
                KIND,
                "longer than 128 bytes",
                &prefix,
            ));
        }
        let valid_segments = prefix.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        });
        if !valid_segments {
            return Err(InvalidServiceValue::new(
                KIND,
                "must be dot-separated segments of ASCII alphanumerics, '-', or '_'",
                &prefix,
            ));
        }
        Ok(Self(prefix))
    }

    /// The validated prefix.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A service-instance discriminator (a port, an identity key, …),
/// appended to every label format.
///
/// Limits: 1–128 bytes, ASCII graphic characters only, and none of
/// `/`, `\`, or `@` (path and systemd-template separators).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceInstance(String);

impl ServiceInstance {
    /// Validate and wrap an instance discriminator.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidServiceValue`] when the instance is empty,
    /// longer than 128 bytes, contains a non-graphic ASCII byte, or
    /// contains `/`, `\`, or `@`.
    pub fn try_new(instance: impl Into<String>) -> Result<Self, InvalidServiceValue> {
        const KIND: &str = "service instance";
        let instance = instance.into();
        if instance.is_empty() {
            return Err(InvalidServiceValue::new(
                KIND,
                "must not be empty",
                &instance,
            ));
        }
        if instance.len() > 128 {
            return Err(InvalidServiceValue::new(
                KIND,
                "longer than 128 bytes",
                &instance,
            ));
        }
        if !instance.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must be ASCII graphic characters",
                &instance,
            ));
        }
        if instance.contains(['/', '\\', '@']) {
            return Err(InvalidServiceValue::new(
                KIND,
                "must not contain '/', '\\', or '@'",
                &instance,
            ));
        }
        Ok(Self(instance))
    }

    /// The validated instance discriminator.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<u16> for ServiceInstance {
    /// A port number is always a valid instance discriminator.
    fn from(port: u16) -> Self {
        Self(port.to_string())
    }
}

macro_rules! string_value_traits {
    ($($ty:ident),+) => {$(
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl std::str::FromStr for $ty {
            type Err = InvalidServiceValue;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_new(s)
            }
        }

        impl AsRef<str> for $ty {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    )+};
}

string_value_traits!(
    ServiceAccountName,
    EnvVarName,
    BinaryName,
    LaunchdLabelPrefix,
    ServiceInstance
);
