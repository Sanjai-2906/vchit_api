use axum::Json;
use oracle::Connection;

use crate::models::GroupModel;

pub async fn get_groups() -> Json<Vec<GroupModel>> {
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

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
