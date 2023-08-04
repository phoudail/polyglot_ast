use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

async fn get_or_create(key: &str, default: &str) -> String {
    sleep(Duration::from_secs(1)).await;
    format!("Key : '{}'", key)
}

async fn get_zipper(tree: &PolyglotZipper) -> String {
    sleep(Duration::from_secs(1)).await;
    format!("Zipper : '{}'", tree)
}

async fn get_polyglot_tree(params: &PolyglotTree) -> String {
    sleep(Duration::from_secs(1)).await;
    format!("PolyglotTree : '{}'", params)
}

async fn eval(tree: &str, expr: &str) -> Result<i32, Box<dyn Error>> {
    sleep(Duration::from_secs(2)).await;
    let result = expr.len() as i32;
    Ok(result)
}

#[tokio::main]
async fn main() {
    match mon_traitement().await {
        Ok(result) => println!("Result : {}", result),
        Err(err) => eprintln!("Error : {}", err),
    }
}

async fn mon_traitement() -> Result<i32, Box<dyn Error>> {
    let value = get_on_create("key", "default_value").await;
    let zipper = get_zipper("zipper").await;
    let tree = get_polyglot_tree("tree").await;
    let result = eval(&tree, "result")?;

    Ok(result)
}
