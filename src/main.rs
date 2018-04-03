#[macro_use]
extern crate stdweb;

use std::rc::Rc;
use std::cell::Cell;

use std::f64::consts::PI;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::{ChangeEvent, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ResizeEvent};

use stdweb::web::html_element::{CanvasElement, InputElement};
use stdweb::web::HtmlElement;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
                $y
        }
    };
}

mod minivec;
use minivec::{angle, distance, magnitude, Point};

fn main() {
    stdweb::initialize();

    let drawing = Rc::new(Cell::new(false));

    let previous = Rc::new(Cell::new(Point {
        x: 0 as f64,
        y: 0 as f64,
    }));

    console!(log, "init");

    let divisions = Rc::new(Cell::new(1));

    let slider: InputElement = document()
        .query_selector("#replication-num")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let slider_label: HtmlElement = document()
        .query_selector("#replication-num-label")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    slider.add_event_listener(
        enclose!( (divisions, slider, slider_label) move |_: ChangeEvent| {
		let new_value = slider.raw_value();
		slider_label.set_text_content(&new_value);
		divisions.set(new_value.parse::<i32>().unwrap());
	}),
    );

    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    let center = Rc::new(Cell::new(Point {
        x: (canvas.offset_width() / 2) as f64,
        y: (canvas.offset_height() / 2) as f64,
    }));

    window().add_event_listener(enclose!( (canvas, center) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
        center.set(Point{
            x: (canvas.offset_width() / 2) as f64,
            y: (canvas.offset_height() / 2) as f64
        });
    }));

    canvas.add_event_listener(
        enclose!( (context, previous, center, drawing, divisions) move |event: MouseMoveEvent| {
            if drawing.get() {
                let prev = previous.get();
                let center = center.get();
				let div = divisions.get();
                let current = Point{x:event.offset_x(),y:event.offset_y()};
                context.move_to(prev.x, prev.y);
                context.line_to(current.x, current.y);

                let delta_prev = distance(prev, center);
                let magnitude_prev = magnitude(delta_prev);
                let theta_prev = angle(delta_prev);

                let delta = distance(current, center);
                let magnitude = magnitude(delta);
                let theta = angle(delta);

				for seg in 0..div {
					context.move_to(
						center.x+(magnitude_prev*(theta_prev+(seg as f64 * (2f64*PI/div as f64))).cos()),
						center.y+(magnitude_prev*(theta_prev+(seg as f64 * (2f64*PI/div as f64))).sin())
						);
					context.line_to(
						center.x+magnitude*(theta+(seg as f64 * (2f64*PI/div as f64))).cos(),
						center.y+magnitude*(theta+(seg as f64 * (2f64*PI/div as f64))).sin()
						);
				}

                /* mirroring
                   context.move_to(((prev.x-center.x)*-1 as f64)+center.x, prev.y);
                   context.line_to(((current.x-center.x)*-1 as f64)+center.x, current.y);
                   */

                context.stroke();
                previous.set(Point{x:current.x,y:current.y});
            }
        }),
    );

    canvas.add_event_listener(
        enclose!( (context, drawing, previous) move |event: MouseDownEvent| {
            drawing.set(true);
            let current = Point{x:event.offset_x(),y:event.offset_y()};
            context.move_to(current.x, current.y);
            previous.set(current);
        }),
    );

    canvas.add_event_listener(enclose!( (drawing) move |_event: MouseUpEvent| {
        drawing.set(false);
    }));

    stdweb::event_loop();
}
