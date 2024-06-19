use russenger::models::RussengerUser;
use russenger::prelude::*;


#[action]
async fn Main(res: Res, req: Req) {
    res.send(TextModel::new(&req.user, "HelloWord")).await;
}

#[russenger::main]
async fn main() {
    let conn = Database::new().await.conn;
    migrate!([RussengerUser], conn);
    russenger::actions![Main];
    russenger::launch().await;
}
