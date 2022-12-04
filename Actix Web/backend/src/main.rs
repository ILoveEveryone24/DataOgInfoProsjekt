use actix_web::{HttpRequest, Error, get, post, web, App, HttpResponse, HttpServer, Responder};
//use actix_web_lab::web::spa;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use actix_files::Files;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};


#[derive(Debug, Serialize, Deserialize)]
struct testingCopy
{
    firstVisit: bool,
    pieces: HashMap<String, (i32, i32, String, bool)>,
}


#[get("/yes")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/john")]
async fn john() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/here")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

//WEBSOCKETS



/// Define HTTP actor
struct MyWs
{
    pieces:HashMap<String, (i32, i32, String, bool)>,
    firstMessage:bool,
    tempoStruct: Arc<Mutex<testingCopy>>//web::Data<testingCopy>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}



/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        println!("Before: {}", self.tempoStruct.lock().unwrap().firstVisit);

        //self.tempoStruct.lock().unwrap().firstVisit = false;

        println!("After: {}", self.tempoStruct.lock().unwrap().firstVisit);

        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {

                // if(self.firstMessage)
                // {
                //     println!("Its a first message");
                //     self.firstMessage = false;
                // }
                // if(!self.firstMessage)
                // {

                // }
                let tmpText:testingCopy = serde_json::from_str(&text).unwrap();
                //let tmpBool = tmpText.pieces == tmpStruct.pieces;
                if(!tmpText.firstVisit)
                {
                    self.tempoStruct.lock().unwrap().pieces = tmpText.pieces;
                    for (key, value) in &self.pieces
                    {
                        if(key == "P3"){
                            println!("Key {}", key);
                            println!("value 0 {}", value.0);
                            println!("value 1 {}", value.1);
                        }
                    
                    }
                    //println!("Is tmptext equal tmpstruct: {}", tmpBool);
                }
                let tmpHash = self.tempoStruct.lock().unwrap().pieces.clone();
                let tmpJson = serde_json::to_string(&testingCopy{firstVisit: false, pieces: tmpHash}).unwrap();
                //self.pieces = tmpStruct.pieces;
                ctx.text(tmpJson)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn json_data(state: web::Data<Mutex<testingCopy>>) -> web::Json<testingCopy>
{
    let tmpState = state.lock().unwrap();
    web::Json(testingCopy{
        firstVisit: false,
        pieces: tmpState.pieces.clone(),
    })
}


async fn index(req: HttpRequest, stream: web::Payload, state: web::Data<Mutex<testingCopy>>) -> Result<HttpResponse, Error> {
    println!("\n\n\nIndex is running\n\n\n");
    let cloneState = Arc::clone(&state);
    let tmpMutex = cloneState.lock().unwrap();
    let tmpStruct = testingCopy{
        firstVisit: tmpMutex.firstVisit.clone(),
        pieces: tmpMutex.pieces.clone(),
    };
    let resp = ws::start(MyWs {pieces: tmpStruct.pieces, firstMessage:true, tempoStruct: state.into_inner()}, &req, stream);
    println!("{:?}", resp);
    resp
}

//WEBSOCKETS END

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = Arc::new(Mutex::new(testingCopy{
        firstVisit:true,
        pieces:HashMap::from(
            [(String::from("p1"), (0, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p2"), (100, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p3"), (200, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p4"), (300, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p5"), (400, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p6"), (500, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p7"), (600, 100, String::from("images/PawnBL.svg"), true)),
            (String::from("p8"), (700, 100, String::from("images/PawnBL.svg"), true)), 

            (String::from("r1"), (0, 0, String::from("images/RookBL.svg"), true)), 
            (String::from("b1"), (200, 0, String::from("images/BishopBL.svg"), true)), 
            (String::from("n1"), (100, 0, String::from("images/KnightBL.svg"), true)), 
            (String::from("q"),  (300, 0, String::from("images/QueenBL.svg"), true)), 
            (String::from("k"),  (400, 0, String::from("images/KingBL.svg"), true)),
            (String::from("r2"), (700, 0, String::from("images/RookBL.svg"), true)), 
            (String::from("b2"), (500, 0, String::from("images/BishopBL.svg"), true)), 
            (String::from("n2"), (600, 0, String::from("images/KnightBL.svg"), true)),

            (String::from("P1"), (0, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P2"), (100, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P3"), (200, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P4"), (300, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P5"), (400, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P6"), (500, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P7"), (600, 600, String::from("images/PawnWH.svg"), true)),
            (String::from("P8"), (700, 600, String::from("images/PawnWH.svg"), true)),

            (String::from("R1"), (700, 700, String::from("images/RookWH.svg"), true)), 
            (String::from("B1"), (500, 700, String::from("images/BishopWH.svg"), true)), 
            (String::from("N1"), (600, 700, String::from("images/KnightWH.svg"), true)), 
            (String::from("Q"),  (300, 700, String::from("images/QueenWH.svg"), true)), 
            (String::from("K"),  (400, 700, String::from("images/KingWH.svg"), true)),
            (String::from("R2"), (0, 700, String::from("images/RookWH.svg"), true)), 
            (String::from("B2"), (200, 700, String::from("images/BishopWH.svg"), true)), 
            (String::from("N2"), (100, 700, String::from("images/KnightWH.svg"), true))]),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .service(hello)
            .service(echo)
            .route("/websocket", web::get().to(index))
            .route("/data", web::get().to(json_data))
            .route("/hey", web::get().to(manual_hello))
            .service(john)
            .service(Files::new("/", "./dist/").index_file("index.html"))
            // .service(
            //     spa()
            //     .index_file("C:/Users/ernes/Desktop/Computer science/Rust/Testing/Testing_2/fullstackrustapp/frontend/dist/index.html")
            //     .static_resources_mount("/static")
            //     .static_resources_location("C:/Users/ernes/Desktop/Computer science/Rust/Testing/Testing_2/fullstackrustapp/frontend/dist")
            //     .finish()
            //     )
            // .service(
            //     spa()
            //     .index_file("C:/Users/ernes/Desktop/Computer science/Rust/Testing/Testing_2/fullstackrustapp/text_frontend/dist/index.html")
            //     .static_resources_mount("/text")
            //     .static_resources_location("C:/Users/ernes/Desktop/Computer science/Rust/Testing/Testing_2/fullstackrustapp/text_frontend/dist")
            //     .finish()
            //     )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}