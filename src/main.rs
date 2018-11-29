#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;

extern crate typed_html;
extern crate typed_html_macros;

use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{get, routes, Request, Response};
use rocket_contrib::serve::{Options, StaticFiles};

// use typed_html::types::LinkType;
use typed_html::dom::DOMTree;
use typed_html::elements::FlowContent;
use typed_html::{html, text};

struct Html(DOMTree<String>);

impl<'r> Responder<'r> for Html {
    fn respond_to(self, _request: &Request) -> Result<'r> {
        Ok(Response::build()
            .status(Status::Ok)
            .header(ContentType::HTML)
            .sized_body(Cursor::new(self.0.to_string()))
            .finalize())
    }
}

fn layout(content: Box<FlowContent<String>>) -> Html {
    Html(html!(
        <html>
            <head>
                <title>"fsr"</title>
            </head>
            <body>
                {content}
            </body>
        </html>
    ))
}

#[get("/")]
fn index() -> Html {
    layout(html!(<h1>"Hello, world"</h1>))
}

fn main() {
    let routes = routes![index];
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/_static");

    rocket::ignite()
        .mount("/static", StaticFiles::new(static_dir, Options::None))
        .mount("/", routes)
        .launch();
}
