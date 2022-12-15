// Derived from the example in nature_of_code

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	// Begin drawing
	let win = app.window_rect();
	let t = app.time;
	let draw = app.draw();

	// Clear the background to blue.
	draw.background().color(BLACK);

	// Create an `ngon` of points.
	let n_points = 5;
	let radius = win.w().min(win.h()) * 0.25;
	let points = (0..n_points).map(|i| {
		let fract = i as f32 / n_points as f32;
		let phase = fract;
		let x = radius * (TAU * phase).cos();
		let y = radius * (TAU * phase).sin();
		pt2(x, y)
	});
	draw.polygon()
		.x(-win.w() * 0.25)
		.color(WHITE)
		.rotate(-t * 0.1)
		.stroke(PINK)
		.stroke_weight(20.0)
		.join_round()
		.points(points);

	// Do the same, but give each point a unique colour.
	let n_points = 7;
	let points_colored = (0..n_points).map(|i| {
		let fract = i as f32 / n_points as f32;
		let phase = fract;
		let x = radius * (TAU * phase).cos();
		let y = radius * (TAU * phase).sin();
		let r = fract;
		let g = 1.0 - fract;
		let b = (0.5 + fract) % 1.0;
		(pt2(x, y), rgb(r, g, b))
	});
	draw.polygon()
		.x(win.w() * 0.25)
		.rotate(t * 0.2)
		.points_colored(points_colored);

	// Write the result of our drawing to the window's frame.
	draw.to_frame(app, &frame).unwrap();
}

// Recursive function
fn draw_circle(draw: &Draw, x: f32, y: f32, r: f32) {
	let norm_radius = map_range(r, 2.0, 360.0, 0.0, 1.0);
	draw.ellipse()
		.x_y(x, y)
		.radius(r)
		.hsva(norm_radius, 0.75, 1.0, norm_radius)
		.stroke(BLACK);

	if r > 8.0 {
		// Four circles! left right, up and down
		draw_circle(&draw, x + r, y, r / 2.0);
		draw_circle(&draw, x - r, y, r / 2.0);
		draw_circle(&draw, x, y + r, r / 2.0);
		draw_circle(&draw, x, y - r, r / 2.0);
	}
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
