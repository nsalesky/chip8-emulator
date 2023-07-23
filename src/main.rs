mod app;

use app::AppModel;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("com.nsalesky.chip8");
    app.run::<AppModel>(());
}
