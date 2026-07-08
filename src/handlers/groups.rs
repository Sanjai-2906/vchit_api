use axum::{Json, extract::State};
use oracle::Connection;

use crate::{AppConfig, models::GroupModel};

pub async fn get_groups(State(config): State<AppConfig>) -> Json<Vec<GroupModel>> {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
    let rows = conn
        .query(
            "select chitbasicid, chitgroupno
             from chitlist
             group by chitbasicid, chitgroupno
             order by 2",
            &[],
        )
        .unwrap();
    let mut groups = Vec::new();
    for row_result in rows {
        let row = row_result.unwrap();

        groups.push(GroupModel {
            group_id: row.get(0).unwrap(),
            group_name: row.get(1).unwrap(),
        });
    }

    return Json(groups);
}
