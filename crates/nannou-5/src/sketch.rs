// https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_blend.rs

use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	// Begin drawing
	let draw = app.draw();

	// Clear the background.
	draw.background().color(BLACK);

	let win = app.window_rect();
	let t = app.time;

	// Decide on a number of points and a weight.
	let n_points = 10;
	let weight = 8.0;
	let hz = ((app.mouse.x + win.right()) / win.w()).powi(4) * 1000.0;
	let vertices = (0..n_points)
			// A sine wave mapped to the range of the window.
			.map(|i| {
					let x = map_range(i, 0, n_points - 1, win.left(), win.right());
					let fract = i as f32 / n_points as f32;
					let amp = (t + fract * hz * TAU).sin();
					let y = map_range(amp, -1.0, 1.0, win.bottom() * 0.75, win.top() * 0.75);
					pt2(x, y)
			})
			.enumerate()
			// Colour each vertex uniquely based on its index.
			.map(|(i, p)| {
					let fract = i as f32 / n_points as f32;
					let r = (t + fract) % 1.0;
					let g = (t + 1.0 - fract) % 1.0;
					let b = (t + 0.5 + fract) % 1.0;
					let rgba = srgba(r, g, b, 1.0);
					(p, rgba)
			});

	// Draw the polyline as a stroked path.
	draw.polyline()
			.weight(weight)
			.join_round()
			.points_colored(vertices);

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
