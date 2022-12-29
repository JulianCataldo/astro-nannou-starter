// https://github.com/nannou-org/nannou/blob/master/examples/draw/draw_blend.rs

use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	// Change the background luminance based on mouse x.
	let w = app.window_rect();
	let lum = map_range(app.mouse.x, w.left(), w.right(), 0.0, 1.0);
	let clear = gray(lum);
	frame.clear(clear);

	// Put all the provided blend modes in a list.
	let blends = [
		("NORMAL", BLEND_NORMAL),
		("ADD", BLEND_ADD),
		("SUBTRACT", BLEND_SUBTRACT),
		("REVERSE SUBTRACT", BLEND_REVERSE_SUBTRACT),
		("DARKEST", BLEND_DARKEST),
		("LIGHTEST", BLEND_LIGHTEST),
	];

	// Select a color blend descriptor based on mouse y.
	let ix = map_range(app.mouse.y, w.top(), w.bottom(), 0, blends.len());
	let ix = std::cmp::min(ix, blends.len() - 1);
	let (blend_name, desc) = &blends[ix];

	// Draw the name of the blend mode and its descriptor.
	let draw = app.draw();
	let color = gray(1.0 - lum.round());
	draw.text(blend_name)
		.color(color)
		.font_size(48)
		.wh(w.wh() * 0.7)
		.align_text_top();
	let text = format!("{:?}", desc);
	draw.text(&text)
		.color(color)
		.wh(w.wh() * 0.8)
		.align_text_bottom();

	// Assign the blend mode.
	let mut draw = draw.color_blend(desc.clone());

	// Draw RGB circles.
	let t = app.time;
	let n_circles = 3;
	let radius = w.right().min(w.top()) * 0.5 / n_circles as f32;
	let animate_radius = -((t.sin() * 0.5 + 0.5) * radius * 0.5);
	draw = draw.x(w.left() * 0.5);
	for i in 0..n_circles {
		let hue = i as f32 / n_circles as f32;
		let color = hsl(hue, 1.0, 0.5);
		draw.ellipse()
			.radius(radius)
			.color(color)
			.x(radius + animate_radius);
		draw = draw.rotate(PI * 2.0 / n_circles as f32);
	}

	// Draw CMY.
	draw = draw.x(w.right() * 0.5);
	for i in 0..n_circles {
		let hue = i as f32 / n_circles as f32;
		let color = hsl(hue + 0.5, 1.0, 0.5);
		draw.ellipse()
			.radius(radius)
			.color(color)
			.x(radius + animate_radius);
		draw = draw.rotate(PI * 2.0 / n_circles as f32);
	}

	// Draw ascending luminance.
	draw = draw.x(w.right() * 0.5);
	for i in 0..n_circles {
		let lum = (0.5 + i as f32) / n_circles as f32;
		let color = gray(lum);
		draw.ellipse()
			.radius(radius)
			.color(color)
			.x(radius + animate_radius);
		draw = draw.rotate(PI * 2.0 / n_circles as f32);
	}

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
