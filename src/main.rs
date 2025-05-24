mod game;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() {
    teleia::run("game", 240, 160, teleia::Options::empty(), game::Game::new).await;
}

#[cfg(target_arch = "wasm32")]
pub fn main() {} // dummy main, real wasm32 main is lib::main_js
