use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{form::Form, State};
use rocket::{Request, Response, Shutdown};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'a>(&self, _req: &'a Request<'_>, _res: &mut Response<'a>) {
        _res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        _res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        _res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        _res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[macro_use]
extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..10))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/msg", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop{
            let msg = select!{
                msg = rx.recv() => match msg {
                    Ok(msg )=> msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,

            };
            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(CORS)
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![index, post, events])
        .mount("/", FileServer::from(relative!("/app/client-files")))
}
