//! Room Survey — an external Open CAD Studio add-on for architectural
//! surveys measured with a tape or a laser meter.
//!
//! The room is first drawn by eye as a closed polyline. The plugin names its
//! corners (A, B, C, …), asks for the measured sides and diagonals, and
//! redraws the room with the true shape: exact when only the minimum is
//! measured (the sides plus `n − 3` diagonals), adjusted by least squares
//! with a residual for every measure when there are more. The result goes on
//! its own layer with dimensions and a table of measures and residuals.
//!
//! Several rooms can be surveyed together, linked by wall thicknesses and
//! measures through the doors, and the result saved as a measures book.
//!
//! The maths ([`solve`]) and the survey model ([`survey`]) are host-free and
//! unit-tested on their own;
//! [`commands`] and [`tools`] are the glue to the host.

use ocs_plugin_api::host::{BuiltinPlugin, HostApi};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
use ocs_plugin_api::ribbon::CadModule;

#[macro_use]
pub mod i18n;
pub mod commands;
pub mod holes;
pub mod labels;
mod ribbon;
pub mod solve;
pub mod survey;
pub mod tools;

/// Reverse-DNS plugin id; also the folder name under `plugins/`.
pub const PLUGIN_ID: &str = "opencad.rilievovani";

// Keep in sync with plugin.toml (the version comes from Cargo.toml).
static MANIFEST: PluginManifest = PluginManifest {
    id: PLUGIN_ID,
    name: "Room Survey",
    version: env!("CARGO_PKG_VERSION"),
    description: "Room survey: adjust a room drawn by eye to the measured sides and diagonals \
                  (least squares), with residuals, dimensions and a table of measures. \
                  English and Italian.",
    api_version: ApiVersion::CURRENT,
    ribbon_order: 61,
    xdata_apps: &[],
    command_prefixes: &["RV_"],
};

struct RilievoPlugin;

impl BuiltinPlugin for RilievoPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }

    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(ribbon::RilievoModule)
    }

    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        commands::handle(host, cmd)
    }
}

ocs_plugin_api::export_plugin!(RilievoPlugin);
