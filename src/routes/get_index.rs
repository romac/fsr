use tide::{Error, Request, Response};
use tide_tera::prelude::*;

use crate::State;

pub async fn get_index(req: Request<State>) -> Result<Response, Error> {
    let state = req.state();
    let data = state.db.as_ref().read(|data| data.clone()).await;

    state
        .tera
        .render_response("index.html", &tera::Context::from_serialize(data)?)
}
