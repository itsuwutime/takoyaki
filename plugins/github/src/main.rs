mod types;
mod plugin;

use plugin::{GithubPlugin, Config};
use takoyaki::takoyaki::Takoyaki;
use takoyaki::plugin::Plugin;

#[tokio::main]
async fn main() {
    let mut takoyaki = Takoyaki::<types::Root , Config>::default();
    let plugin = GithubPlugin::new();

    takoyaki.plug(&plugin);

    takoyaki.start().await.unwrap()
}

