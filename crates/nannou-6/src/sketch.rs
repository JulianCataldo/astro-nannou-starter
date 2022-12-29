// https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_blend.rs

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

	// Clear the background to black.
	draw.background().color(BLACK);

	// Use the mouse position to affect the frequency and amplitude.
	let hz = map_range(app.mouse.x, win.left(), win.right(), 0.0, 100.0);
	let amp = app.mouse.y;

	// Create an iterator yielding triangles for drawing a sine wave.
	let tris = (1..win.w() as usize)
			.flat_map(|i| {
					let l_fract = (i - 1) as f32 / win.w();
					let r_fract = i as f32 / win.w();

					// Map the vertices to the window.
					let l_x = map_range(l_fract, 0.0, 1.0, win.left(), win.right());
					let r_x = map_range(r_fract, 0.0, 1.0, win.left(), win.right());
					let l_y = (t * hz + l_fract * hz * TAU).sin() * amp;
					let r_y = (t * hz + r_fract * hz * TAU).sin() * amp;

					// Produce this slice of the triangle as a quad.
					let a = pt2(l_x, l_y);
					let b = pt2(r_x, r_y);
					let c = pt2(r_x, 0.0);
					let d = pt2(l_x, 0.0);
					geom::Quad([a, b, c, d]).triangles_iter()
			})
			.map(|tri| {
					// Color the vertices based on their amplitude.
					tri.map_vertices(|v| {
							let y_fract = map_range(v.y.abs(), 0.0, win.top(), 0.0, 1.0);
							let color = srgba(y_fract, 1.0 - y_fract, 1.0 - y_fract, 1.0);
							(v.extend(0.0), color)
					})
			});

	// Draw the mesh!
	draw.mesh().tris_colored(tris);

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
