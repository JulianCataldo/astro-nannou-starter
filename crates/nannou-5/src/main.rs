// native app entry_point

mod sketch;
use async_std::task::block_on;
use sketch::{run_app, Model};
fn main() {
	let model = Model {};
	block_on(async {
		run_app(model).await;
	});
}
