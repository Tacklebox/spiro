#[macro_use]
extern crate stdweb;

use std::rc::Rc;
use std::cell::Cell;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d
};

use stdweb::web::event::{
    MouseMoveEvent,
    MouseDownEvent,
    MouseUpEvent,
    ResizeEvent,
};

use stdweb::web::html_element::CanvasElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}


fn main() {
    stdweb::initialize();
    let drawing = Rc::new(Cell::new(false));
    let drawing_md = drawing.clone();
    let drawing_mu = drawing.clone();
    console!(log, "init");
    let canvas: CanvasElement = document()
        .query_selector( "#canvas" )
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    canvas.add_event_listener( enclose!( (context) move |event: MouseMoveEvent| {
        if drawing.get() {
            context.fill_rect(f64::from(event.client_x() - 5), f64::from(event.client_y() - 5)
                              , 10.0, 10.0);
        }
    }));

    canvas.add_event_listener( move |_event: MouseDownEvent| {
        drawing_md.set(true);
    });

    canvas.add_event_listener( move |_event: MouseUpEvent| {
        drawing_mu.set(false);
    });

    stdweb::event_loop();
}
