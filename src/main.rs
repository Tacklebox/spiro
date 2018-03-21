#[macro_use]
extern crate stdweb;

use std::rc::Rc;
use std::cell::Cell;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::{MouseDownEvent, MouseMoveEvent, MouseUpEvent, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    stdweb::initialize();

    let drawing = Rc::new(Cell::new(false));

    let previous = Rc::new(Cell::new(Point {
        x: 0 as f64,
        y: 0 as f64,
    }));

    console!(log, "init");

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
        enclose!( (context, previous, center, drawing) move |event: MouseMoveEvent| {
        if drawing.get() {
            let prev = previous.get();
			let center = center.get();
            let user_x = f64::from(event.client_x());
            let user_y = f64::from(event.client_y());
            context.move_to(prev.x, prev.y);
            context.line_to(user_x, user_y);
            context.move_to(((prev.x-center.x)*-1 as f64)+center.x, prev.y);
            context.line_to(((user_x-center.x)*-1 as f64)+center.x, user_y);
            context.stroke();
            previous.set(Point{x:user_x,y:user_y});
        }
    }),
    );

    canvas.add_event_listener(
        enclose!( (context, drawing, previous) move |event: MouseDownEvent| {
        drawing.set(true);
        let user_x = f64::from(event.client_x());
        let user_y = f64::from(event.client_y());
        previous.set(Point{x:user_x,y:user_y});
        context.move_to(user_x, user_y);
    }),
    );

    canvas.add_event_listener(enclose!( (drawing) move |_event: MouseUpEvent| {
        drawing.set(false);
    }));

    stdweb::event_loop();
}
