use axum::{Json, extract::Path};
use oracle::Connection;

use crate::models::MemberModel;
pub async fn get_members(grp_name: Option<Path<String>>) -> Json<Vec<MemberModel>> {
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

    let mut members = Vec::new();

    let rows = match grp_name {
        Some(Path(grp)) => conn
            .query(
                "select partymastid, partyid
                 from chitlist
                 where chitgroupno = :1
                 group by partymastid, partyid
                 order by 2",
                &[&grp],
            )
            .unwrap(),

        None => conn
            .query(
                "select partymastid, partyid
                 from chitlist",
                &[],
            )
            .unwrap(),
    };

    // for row_result in rows {
    //     let row = row_result.unwrap();

    //     members.push(MemberModel {
    //         member_id: row.get(0).unwrap(),
    //         member_name: row.get(1).unwrap(),
    //     });
    // }
    for row_result in rows {
        let row = row_result.unwrap();

        let member_id: i64 = row.get(0).unwrap();

        let member_name: Option<String> = row.get(1).unwrap();

        if let Some(name) = member_name {
            members.push(MemberModel {
                member_id,
                member_name: name,
            });
        }
    }

    return Json(members);
}
