use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 3000;

    println!("Serving on http://localhost:{}", port);
    HttpServer::new(|| App::new().service(get_index).service(post_gcd))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <h1>GCD Calculator</h1>
            <form action="/gcd" method="post">
                <input required type="number" min="0" name="n" />
                <input required type="number" min="0" name="m" />
                <button type="submit">Compute</button>
            </form>
        "#,
    )
}

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zeroes is boring");
    }

    let body = format!(
        "The greatest common divisor of the numbers {} and {} is <strong>{}</strong>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(body)
}
