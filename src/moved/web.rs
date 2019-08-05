//use actix_web::middleware;
//use actix_web::web;
//use actix_web::web::Payload;
//use actix_web::App;
//use actix_web::Error;
//use actix_web::HttpResponse;
//use actix_web::HttpServer;
//use bytes::BytesMut;
//use futures::prelude::*;

//    #[structopt(short = "p", long = "port")]
//    port: Option<u16>,

//    if let Some(port) = args.port {
//        let address = format!("0.0.0.0:{}", port);
//        info!("Starting the HTTP server at {}", address);
//
//        HttpServer::new(|| {
//            App::new()
//                .wrap(middleware::Logger::default())
//                .service(web::resource("/compile").to_async(handler))
//        })
//        .bind(address)
//        .expect("Server binding error")
//        .run()
//        .expect("Server running error");
//    }

//#[derive(Serialize)]
//struct CircuitError {
//    error: String,
//}
//
//fn handler(payload: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
//    payload
//        .from_err()
//        .fold(BytesMut::new(), move |mut body, chunk| {
//            body.extend_from_slice(&chunk);
//            Ok::<_, Error>(body)
//        })
//        .and_then(|body| {
//            let code = String::from_utf8_lossy(&body.to_vec()).to_string();
//            info!("Received: {:?}", code);
//
//            let response = match compiler::compile(&code) {
//                Ok(circuit) => serde_json::to_vec(&circuit),
//                Err(error) => serde_json::to_vec(&CircuitError {
//                    error: error.to_string(),
//                }),
//            }
//                .expect("Serialization bug");
//
//            HttpResponse::Ok().body(response)
//        })
//}
