//! Steward Abscissa Application

use crate::{commands::EntryPoint, config::StewardConfig};
use abscissa_core::{
    application::{self, AppCell},
    config::{self, CfgCell},
    trace, Application, FrameworkError, StandardPaths,
};

/// Application state
pub static APP: AppCell<StewardApp> = AppCell::new();

/// Steward Application
#[derive(Debug)]
pub struct StewardApp {
    /// Application configuration.
    config: CfgCell<StewardConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for StewardApp {
    fn default() -> Self {
        Self {
            config: CfgCell::default(),
            state: application::State::default(),
        }
    }
}

impl Application for StewardApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint;

    /// Application configuration.
    type Cfg = StewardConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<StewardConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    ///
    /// If you would like to add additional components to your application
    /// beyond the default ones provided by the framework, this is the place
    /// to do so.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let mut framework_components = self.framework_components(command)?;
        let mut app_components = self.state.components_mut();
        framework_components.push(Box::new(abscissa_tokio::TokioComponent::new()?));
        app_components.register(framework_components)
    }

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        let mut components = self.state.components_mut();
        components.after_config(&config)?;
        self.config.set_once(config);

        // Start metrics server
        tokio::spawn(async move {
            if let Err(e) = crate::start_metrics_server().await.await {
                abscissa_core::tracing::error!("Failed to start metrics server: {}", e);
            }
        });

        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &EntryPoint) -> trace::Config {
        if command.verbose {
            trace::Config::verbose()
        } else {
            match std::env::var("RUST_LOG") {
                Ok(val) => {
                    if !val.is_empty() {
                        val.into()
                    } else {
                        trace::Config::default()
                    }
                }
                Err(_) => trace::Config::default(),
            }
        }
    }
}
