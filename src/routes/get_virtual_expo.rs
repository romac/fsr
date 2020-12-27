use tide::{Error, Request, Response};
use tide_tera::prelude::*;

use crate::State;

pub async fn get_virtual_expo(req: Request<State>) -> Result<Response, Error> {
    let state = req.state();
    let data = state.db.as_ref().read(|data| data.clone()).await;

    state
        .tera
        .render_response("virtual_expo.html", &tera::Context::from_serialize(data)?)
}
