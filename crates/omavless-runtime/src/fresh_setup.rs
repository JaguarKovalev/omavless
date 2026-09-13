// SPDX-License-Identifier: MIT

//! Explicit create-only preparation for a new native user's private config.
//! No service, socket client, ownership publication, network, or Python calls.
//! This is not activation and never repairs an existing installation.

use crate::cutover::{CutoverError, CutoverPaths, MigrationLock};
use crate::store_bootstrap::{
    EMPTY_STORE_PAYLOAD, PrivateStoreBootstrapOutcome, PrivateStoreBootstrapPaths,
    bootstrap_private_store_locked,
};
use nix::unistd::Uid;
use omavless_store::{PrivateCreateOutcome, atomic_create_private, read_private_utf8};
use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::os::unix::fs::{DirBuilderExt, MetadataExt};
use std::path::{Component, Path, PathBuf};

const TEMPLATE_NAME: &str = "route-template.yaml";
const TEMPLATE: &[u8] = include_bytes!("../../../templates/default.yaml");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    UnsafePath,
    ExistingInstallation,
    Busy,
    Io,
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsafePath => "Native setup paths or permissions are unsafe",
            Self::ExistingInstallation => "Native setup requires unused state and unchanged initial config; existing data was not replaced",
            Self::Busy => "Another OmaVLESS operation owns the migration lock",
            Self::Io => "Native setup could not finish; no activation was attempted",
        })
    }
}
impl std::error::Error for SetupError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetupOutcome {
    pub created_files: u8,
}

// No Debug: configured filesystem roots are private local account metadata.
struct SetupPaths {
    home: PathBuf,
    cutover: CutoverPaths,
}

fn validate_chain(path: &Path) -> Result<(), SetupError> {
    if !path.is_absolute()
        || path.as_os_str().len() > 4096
        || path.to_str().is_none()
        || path
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .any(u8::is_ascii_control)
        || path
            .components()
            .any(|p| !matches!(p, Component::RootDir | Component::Normal(_)))
    {
        return Err(SetupError::UnsafePath);
    }
    let mut current = PathBuf::new();
    for part in path.components() {
        current.push(part);
        match fs::symlink_metadata(&current) {
            Ok(m) if m.is_dir() && !m.file_type().is_symlink() => {}
            Err(e) if e.kind() == ErrorKind::NotFound => break,
            _ => return Err(SetupError::UnsafePath),
        }
    }
    Ok(())
}

fn directory(path: &Path, uid: u32, private: bool) -> Result<bool, SetupError> {
    validate_chain(path)?;
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
        Ok(m)
            if m.is_dir()
                && m.uid() == uid
                && m.mode() & 0o022 == 0
                && (!private || m.mode() & 0o777 == 0o700) =>
        {
            Ok(true)
        }
        _ => Err(SetupError::UnsafePath),
    }
}

fn ensure_directory(path: &Path, uid: u32, private: bool) -> Result<(), SetupError> {
    if !directory(path, uid, private)? {
        match fs::DirBuilder::new().mode(0o700).create(path) {
            Ok(()) => {
                File::open(path.parent().ok_or(SetupError::UnsafePath)?)
                    .and_then(|p| p.sync_all())
                    .map_err(|_| SetupError::Io)?;
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
            Err(_) => return Err(SetupError::Io),
        }
    }
    directory(path, uid, private)?
        .then_some(())
        .ok_or(SetupError::UnsafePath)
}

fn unused_state(paths: &SetupPaths, uid: u32) -> Result<(), SetupError> {
    let state = &paths.cutover.state_directory;
    if directory(state, uid, true)?
        && fs::read_dir(state)
            .map_err(|_| SetupError::Io)?
            .next()
            .is_some()
    {
        return Err(SetupError::ExistingInstallation);
    }
    match fs::symlink_metadata(paths.cutover.runtime_base.join("omavless")) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        _ => Err(SetupError::ExistingInstallation),
    }
}

fn expected_member(path: &Path, expected: &[u8], uid: u32) -> Result<bool, SetupError> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
        Ok(m)
            if m.is_file()
                && !m.file_type().is_symlink()
                && m.uid() == uid
                && m.mode() & 0o777 == 0o600
                && m.len() == expected.len() as u64 =>
        {
            let text =
                read_private_utf8(path, uid).map_err(|_| SetupError::ExistingInstallation)?;
            (text.as_bytes() == expected)
                .then_some(true)
                .ok_or(SetupError::ExistingInstallation)
        }
        _ => Err(SetupError::ExistingInstallation),
    }
}

fn initial_config(config: &Path, uid: u32) -> Result<(), SetupError> {
    if directory(config, uid, true)? {
        // Stop at the first unexpected member; no unbounded directory scan.
        for entry in fs::read_dir(config).map_err(|_| SetupError::Io)?.take(3) {
            let name = entry.map_err(|_| SetupError::Io)?.file_name();
            if name != "profiles.json" && name != TEMPLATE_NAME {
                return Err(SetupError::ExistingInstallation);
            }
        }
        expected_member(&config.join("profiles.json"), EMPTY_STORE_PAYLOAD, uid)?;
        expected_member(&config.join(TEMPLATE_NAME), TEMPLATE, uid)?;
    }
    Ok(())
}

fn prepare(paths: &SetupPaths, uid: u32) -> Result<SetupOutcome, SetupError> {
    if !directory(&paths.home, uid, false)? {
        return Err(SetupError::UnsafePath);
    }
    validate_chain(&paths.cutover.state_directory)?;
    let state_base = paths
        .cutover
        .state_directory
        .parent()
        .ok_or(SetupError::UnsafePath)?;
    let default_state = paths.home.join(".local/state");
    if state_base == default_state {
        directory(&paths.home.join(".local"), uid, false)?;
        directory(state_base, uid, false)?;
    } else if !directory(state_base, uid, false)? {
        // Custom XDG roots are host configuration, not arbitrary mkdir inputs.
        return Err(SetupError::UnsafePath);
    }
    if !directory(&paths.cutover.runtime_base, uid, true)? {
        return Err(SetupError::UnsafePath);
    }
    let parent = paths.home.join(".config");
    let config = parent.join("omavless");
    directory(&parent, uid, false)?;
    unused_state(paths, uid)?;
    let lock = MigrationLock::acquire(&paths.cutover, uid).map_err(|e| match e {
        CutoverError::Busy => SetupError::Busy,
        _ => SetupError::UnsafePath,
    })?;
    unused_state(paths, uid)?;
    initial_config(&config, uid)?;
    if state_base == default_state {
        ensure_directory(&paths.home.join(".local"), uid, false)?;
        ensure_directory(state_base, uid, false)?;
    }
    ensure_directory(&parent, uid, false)?;
    ensure_directory(&config, uid, true)?;
    initial_config(&config, uid)?;

    let template = config.join(TEMPLATE_NAME);
    let created_template = if expected_member(&template, TEMPLATE, uid)? {
        false
    } else {
        atomic_create_private(&template, TEMPLATE, uid).map_err(|_| SetupError::Io)?
            == PrivateCreateOutcome::Created
    };
    if !expected_member(&template, TEMPLATE, uid)? {
        return Err(SetupError::Io);
    }
    let created_store = bootstrap_private_store_locked(
        &PrivateStoreBootstrapPaths::below_home(&paths.home),
        &paths.cutover,
        uid,
        &lock,
    )
    .map_err(|_| SetupError::Io)?
        == PrivateStoreBootstrapOutcome::Created;
    initial_config(&config, uid)?;
    if !expected_member(&config.join("profiles.json"), EMPTY_STORE_PAYLOAD, uid)?
        || !expected_member(&template, TEMPLATE, uid)?
    {
        return Err(SetupError::Io);
    }
    unused_state(paths, uid)?;
    Ok(SetupOutcome {
        created_files: u8::from(created_template) + u8::from(created_store),
    })
}

/// Prepare only fixed defaults. Does not claim host readiness or native ownership.
pub fn prepare_current() -> Result<SetupOutcome, SetupError> {
    if env::var_os("OMAVLESS_HOME").is_some() {
        return Err(SetupError::UnsafePath);
    }
    let uid = Uid::current().as_raw();
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or(SetupError::UnsafePath)?;
    let cutover = CutoverPaths::current(uid).map_err(|_| SetupError::UnsafePath)?;
    prepare(&SetupPaths { home, cutover }, uid)
}
