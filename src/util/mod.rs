mod assets;
mod loadable;

pub use loadable::load_loadables;

pub mod prelude {
    pub use super::assets::{AssetProvider, LoadingStateExt};
    pub use super::loadable::{AppLoadableExt, Loadable};
}
