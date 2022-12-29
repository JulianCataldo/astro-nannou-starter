use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	// Begin drawing
	let draw = app.draw();

	// Clear the background to blue.
	draw.background().color(CORNFLOWERBLUE);

	// Draw a purple triangle in the top left half of the window.
	let win = app.window_rect();
	draw.tri()
		.points(win.bottom_left(), win.top_left(), win.top_right())
		.color(VIOLET);

	// Draw an ellipse to follow the mouse.
	let t = app.time;
	draw.ellipse()
		.x_y(app.mouse.x * t.cos(), app.mouse.y)
		.radius(win.w() * 0.125 * t.sin())
		.color(RED);

	// Draw a line!
	draw.line()
		.weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
		.caps_round()
		.color(PALEGOLDENROD)
		.points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

	// Draw a quad that follows the inverse of the ellipse.
	draw.quad()
		.x_y(-app.mouse.x, app.mouse.y)
		.color(DARKGREEN)
		.rotate(t);

	// Draw a rect that follows a different inverse of the ellipse.
	draw.rect()
		.x_y(app.mouse.y, app.mouse.x)
		.w(app.mouse.x * 0.25)
		.hsv(t, 1.0, 1.0);

	// Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app(model: Model) {
	// Since ModelFn is not a closure we need this workaround to pass the calculated model
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| m.borrow_mut().take().unwrap())
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.title("nannou web test")
		// .raw_event(raw_event)
		// .key_pressed(key_pressed)
		// .key_released(key_released)
		// .mouse_pressed(mouse_pressed)
		// .mouse_moved(mouse_moved)
		// .mouse_released(mouse_released)
		// .mouse_wheel(mouse_wheel)
		// .touch(touch)
		.view(view)
		.build_async()
		.await
		.unwrap();
}
