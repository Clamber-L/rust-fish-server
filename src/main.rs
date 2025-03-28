use axum::Router;
use rust_fish_server::init_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let secret = "mysql://root:Lsw%400516@47.95.179.146:3306/fish";
    println!("config:{:?}", secret);

    let router: Router = init_router(&secret).await?;
    Ok(router.into())
}
