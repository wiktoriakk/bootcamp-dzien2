#[ic_cdk::query]
fn greet(name: String, surname: i8) -> String {
    format!("Hello, {} {}!", name, surname)
}
