use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // ساخت برنامه با routeهای مختلف
    let app = Router::new()
        .route("/", get(|| async { "🎉 سلام دنیا از Axum!" }))
        .route("/salam", get(|| async { "سلام فارسی! 🇮🇷" }))
        .route("/health", get(|| async { "✅ سرور کار میکند!" }));

    // اجرای سرور
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 سرور Axum شروع شد: http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}