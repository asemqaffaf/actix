use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct Res {
    foo: String,
}
#[get("/")]
async fn hello() -> impl Responder {
    let res = Res {
        foo: "Hello world!".to_string(),
    };
    HttpResponse::Ok().json(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("num_cpus :>>>");
    println!("{}", num_cpus::get());

    let port = env::var("PORT")
        .ok()
        .and_then(|s: String| s.parse().ok())
        .unwrap_or(3000);

    println!("http://127.0.0.1:{}", port);

    HttpServer::new(|| App::new().service(hello).service(echo))
        .workers(num_cpus::get())
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use serde_json;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let res: Res = serde_json::from_slice(&body).unwrap();
        assert_eq!(res.foo, "Hello world!");
    }

    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(App::new().service(echo)).await;

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload("test payload")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "test payload");
    }
}
