use boa_engine::{Context, Source};

#[tokio::main]
async fn main() {
    let j1 = tokio::spawn(worker());
    tokio::try_join!(j1).unwrap();
}

async fn worker() {
    let context = make_context();
    let boa_context = tokio::sync::RwLock::new(context);
    some_worker_func(&boa_context).await;
}

pub fn make_context<'context>() -> Context<'context> {
    let mut context = Context::default();
    let src2 = Source::from_bytes(r#"var two = 2;"#);
    let _ = context.eval(src2).unwrap();
    context
}

pub async fn some_worker_func(
    boa_context: &tokio::sync::RwLock<boa_engine::Context<'_>>,
) -> () {
    println!("do nothing");
}
