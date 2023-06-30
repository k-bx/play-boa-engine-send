use boa_engine::{Context, Source};

#[tokio::main]
async fn main() {
    worker().await;
}

async fn worker() {
    let mut context = Context::default();
    let src2 = Source::from_bytes(r#"var two = 2;"#);
    let _ = context.eval(src2).unwrap();
}
