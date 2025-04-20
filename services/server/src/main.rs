use axum::Router;
use server::init_router;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret = secrets.get("MYSQL_URL").expect("secret was not found");
    println!("config:{:?}", secret);

    let router: Router = init_router(&secret).await?;
    Ok(router.into())
}
