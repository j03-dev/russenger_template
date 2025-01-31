use russenger::models::RussengerUser;
use russenger::prelude::*;


async fn index(res: Res, req: Req) -> Result<()> {
    res.send(TextModel::new(&req.user, "HelloWord")).await?;
    Ok(())
}

#[russenger::main]
async fn main() -> Result<()> {
    let conn = Database::new().await?.conn;
    migrate!([RussengerUser], &conn);

    App::new().await?
        .attach(Router::new().add("/", index))
        .launch()
        .await?

    Ok(())
}
