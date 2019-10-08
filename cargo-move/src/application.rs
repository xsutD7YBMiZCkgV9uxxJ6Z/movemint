//! CargoMove Abscissa Application

use crate::{commands::CargoMoveCmd, config::CargoMoveConfig};
use abscissa_core::{
    application, config, logging, Application, EntryPoint, FrameworkError, StandardPaths,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::Lock<CargoMoveApp> = application::Lock::default();
}

/// Obtain a read-only (multi-reader) lock on the application state.
///
/// Panics if the application state has not been initialized.
pub fn app_reader() -> application::lock::Reader<CargoMoveApp> {
    APPLICATION.read()
}

/// Obtain an exclusive mutable lock on the application state.
pub fn app_writer() -> application::lock::Writer<CargoMoveApp> {
    APPLICATION.write()
}

/// Obtain a read-only (multi-reader) lock on the application configuration.
///
/// Panics if the application configuration has not been loaded.
pub fn app_config() -> config::Reader<CargoMoveApp> {
    config::Reader::new(&APPLICATION)
}

/// CargoMove Application
#[derive(Debug)]
pub struct CargoMoveApp {
    /// Application configuration.
    config: Option<CargoMoveConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for CargoMoveApp {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for CargoMoveApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint<CargoMoveCmd>;

    /// Application configuration.
    type Cfg = CargoMoveConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> &CargoMoveConfig {
        self.config.as_ref().expect("config not loaded")
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut application::State<Self> {
        &mut self.state
    }

    /// Register all components used by this application.
    ///
    /// If you would like to add additional components to your application
    /// beyond the default ones provided by the framework, this is the place
    /// to do so.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// Get logging configuration from command-line options
    fn logging_config(&self, command: &EntryPoint<CargoMoveCmd>) -> logging::Config {
        if command.verbose {
            logging::Config::verbose()
        } else {
            logging::Config::default()
        }
    }
}
