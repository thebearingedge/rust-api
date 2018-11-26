use std::{
    error::Error,
    io::Read,
    sync::{Arc, Mutex},
};

use crate::database::Database;
use crate::models::Post;
use iron::{headers::ContentType, status, AfterMiddleware, Handler, IronResult, Request, Response};
use router::Router;
use serde_json;
use uuid::Uuid;

macro_rules! handle_json {
    ($payload: expr) => {
        match $payload {
            Ok(payload) => payload,
            Err(err) => {
                return Ok(Response::with((
                    status::InternalServerError,
                    err.description(),
                )))
            }
        }
    };
    ($payload: expr, $status: expr) => {
        match $payload {
            Ok(value) => value,
            Err(err) => return Ok(Response::with(($status, err.description()))),
        }
    };
}

macro_rules! lock {
    ($mutex: expr) => {
        $mutex.lock().unwrap()
    };
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => match router.find($e) {
                Some(v) => v,
                None => return Ok(Response::with(status::BadRequest)),
            },
            None => return Ok(Response::with(status::InternalServerError)),
        }
    };
}

pub struct Handlers {
    pub find: Find,
    pub create: Create,
    pub find_by_id: FindById,
}

impl Handlers {
    pub fn new(db: Database) -> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            find: Find::new(database.clone()),
            create: Create::new(database.clone()),
            find_by_id: FindById::new(database.clone()),
        }
    }
}

pub struct Find {
    database: Arc<Mutex<Database>>,
}

impl Find {
    fn new(database: Arc<Mutex<Database>>) -> Find {
        Find { database }
    }
}

impl Handler for Find {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        let db = lock!(self.database);
        let payload = handle_json!(serde_json::to_string(db.get_posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct Create {
    database: Arc<Mutex<Database>>,
}

impl Create {
    fn new(database: Arc<Mutex<Database>>) -> Create {
        Create { database }
    }
}

impl Handler for Create {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        handle_json!(req.body.read_to_string(&mut payload));
        let post = handle_json!(serde_json::from_str(&payload), status::BadRequest);
        let mut db = lock!(self.database);
        db.add_post(post);
        Ok(Response::with((status::Created, payload)))
    }
}

pub struct FindById {
    database: Arc<Mutex<Database>>,
}

impl FindById {
    fn new(database: Arc<Mutex<Database>>) -> FindById {
        FindById { database }
    }

    fn find_post(&self, post_id: &Uuid) -> Option<Post> {
        let db = lock!(self.database);
        db.get_posts()
            .iter()
            .find(|p| p.post_id() == post_id)
            .map(|p| p.clone())
    }
}

impl Handler for FindById {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let post_id = get_http_param!(req, "post_id");
        let post_id = handle_json!(Uuid::parse_str(post_id), status::BadRequest);
        if let Some(post) = self.find_post(&post_id) {
            let payload = handle_json!(serde_json::to_string(&post), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with((status::NotFound, "{}")))
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}
