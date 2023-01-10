use graphul::{extract::Json, http::Methods, Context, Graphul};
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async { "Hello, World 👋!" });

    app.get("/greet/:name", |c: Context| async move {
        let name = c.params("name");
        let country = c.query("country");
        let ip = c.ip();

        format!("My name is {name}, I'm from {country}, my IP is {ip}",)
    });

    app.get("/json", || async {
        Json(json!({
            "name": "full_name",
            "age": 98,
            "phones": [
                format!("+44 {}", 8)
            ]
        }))
    });

    app.run("127.0.0.1:8000").await;
}
