use std::error::Error;
use log::error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let result = todo::run().await;
    if let Err(e) = result.as_ref() {
        error!("{:?}", e)
    }
    result
}

