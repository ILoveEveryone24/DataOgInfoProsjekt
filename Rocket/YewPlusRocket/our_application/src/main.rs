#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{relative, FileServer};
use rocket::{State, Shutdown};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, Receiver, error::RecvError};
use rocket::tokio::select;
//use serde::{Serialize, Deserialize};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::Redirect;
use tokio_postgres::{NoTls, Error};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}





#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct piecesLayout
{
    pieces: Mutex<HashMap<String, (i32, i32, String, bool)>>,
}


struct piecesLayoutCopy
{
    pieces: Arc<Mutex<HashMap<String, (i32, i32, String, bool)>>>,
}


#[get("/data")]
async fn data_json() -> Json<piecesLayout>
{
    let sharedCopy: State<piecesLayoutCopy> = State::from(*shared.inner());
    Json(piecesLayout{pieces: Arc::try_unwrap(sharedCopy.pieces).unwrap()})
    }

#[get("/")]
fn redir() -> Redirect
{
    Redirect::to(uri!(john()))
}

#[get("/john", format="text/html")]
async fn john() -> Template
{
    let context = context! {};
    Template::render("wasm", context)
}

#[post("/here", data="<data>")]
async fn john_post(data:&str, shared: &State<piecesLayoutCopy>) 
{
    let deser:piecesLayout = serde_json::from_str(&data).unwrap();
    //let _res = db_handler(deser).await;

    let mut locked = shared.inner();//.lock().expect("didnt work");
    //let mut deserCopy = Mutex::new(deser).lock().expect("Whoops");
    locked = &piecesLayoutCopy{pieces:Arc::from(Mutex::from(deser.pieces))};
}


// async fn db_handler(data:piecesLayoutCopy) -> Result<(), Error> {
//     let (client, connection) = tokio_postgres::connect("postgres://postgres:postgres@localhost:5432", NoTls).await?;
   
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("Connection error: {}", e);
//         }
//     });

//     println!("Connected to the database!");

    

//     client.batch_execute("
//         CREATE TABLE IF NOT EXISTS chess_moves(
//             id SERIAL PRIMARY KEY,
//             pieceId VARCHAR(10) NOT NULL,
//             xPos INT NOT NULL,
//             yPos INT NOT NULL,
//             imgLink VARCHAR(40) NOT NULL,
//             existance BOOL NOT NULL
//         )
//         ").await?;

//     for (key, value) in data.pieces
//     {
//         let pieceId = key;
//         let (xPos, yPos, imgLink, existance) = value;

//         client.execute(
//             "INSERT INTO chess_moves (pieceId, xPos, yPos, imgLink, existance) VALUES ($1, $2, $3, $4, $5)",
//             &[&pieceId, &xPos, &yPos, &imgLink, &existance],
//         ).await?;
//     }
        
//     println!("Added to database");
//     Ok(())
// }



 // async fn db_return() -> Result<piecesLayoutCopy, Error> {
 //    let (client, connection) = tokio_postgres::connect("postgres://postgres:postgres@localhost:5432", NoTls).await?;
   
 //    tokio::spawn(async move {
 //         if let Err(e) = connection.await {
 //             eprintln!("Connection error: {}", e);
 //         }
 //    });

 //    println!("Connected to the database!");

 //    //let numRows = client.query("SELECT COUNT(*) FROM chess_moves", &[]).await?;

 //    let mut checkVal = false;
 //    while !checkVal
 //    {
 //        let checkReturn = client.query("SELECT EXISTS (SELECT 1 FROM chess_moves WHERE id = $1)", &[&32]).await?;
 //        checkVal = checkReturn[0].get(0);
 //    }

 //    let moveInfo = client.query("SELECT * FROM chess_moves", &[]).await?;
 //    let mut piecesReturned:HashMap<String, (i32, i32, String, bool)> = HashMap::new();



 //    for n in 0..32
 //    {
 //        let valueTwo:&str =  moveInfo[n].get(1);
 //        let valueThree:i32 =  moveInfo[n].get(2);
 //        let valueFour:i32 =  moveInfo[n].get(3);
 //        let valueFive:&str =  moveInfo[n].get(4);
 //        let valueSix:bool =  moveInfo[n].get(5);

 //        piecesReturned.insert(valueTwo.to_string(), (valueThree, valueFour, valueFive.to_string(), valueSix));
 //    }
    
 //    client.query("TRUNCATE TABLE chess_moves", &[]).await?;
 //    client.query("ALTER SEQUENCE chess_moves_id_seq RESTART", &[]).await?;

 //    println!("Handling done and table truncated");

 //    let tmpStruct = piecesLayoutCopy{moveId: 0, pieces: piecesReturned};
 //    Ok(tmpStruct)
 // }


#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(piecesLayoutCopy{pieces:Arc::new(Mutex::new(HashMap::new()))})
    .mount("/", routes![hello, john, john_post, redir, data_json])
    .mount("/assets", FileServer::from(r"C:\Users\ernes\Desktop\Computer science\Rust\Testing\Testing_2\YewPlusRocket\our_application\static\"))
    .mount("/images", FileServer::from(r"C:\Users\ernes\Desktop\Computer science\Rust\Testing\Testing_2\YewPlusRocket\our_application\static\images\"))
    //.mount("/js", FileServer::from(r"C:\Users\ernes\Desktop\Computer science\Rust\Testing\Testing_2\YewPlusRocket\our_application\js\"))
    .attach(Template::fairing())
}