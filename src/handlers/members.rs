use axum::{Json, extract::Path};
use oracle::Connection;

use crate::models::MemberModel;
pub async fn get_members(grp_name: Option<Path<String>>) -> Json<Vec<MemberModel>> {
    println!("Group Name: {:?}",grp_name);
    let conn = Connection::connect("vvcpl", "log", "velcloud.in:1521/XE").unwrap();

    let mut members = Vec::new();

    let rows = match grp_name {
        Some(Path(grp)) => conn
            .query(
                "SELECT
                        PARTYMASTID,
                        PARTYID,
                        MAX(MOBILE) AS MOBILE
                    FROM CHITLIST
                    WHERE CHITGROUPNO = :1
                    GROUP BY PARTYMASTID, PARTYID
                    ORDER BY PARTYID",
                &[&grp],
            )
            .unwrap(),

        None => conn
            .query(
                "select partymastid, partyid, mobile
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
        
        let mobile: Option<String> = row.get(2).unwrap();

        if let Some(name) = member_name {
            members.push(MemberModel {
                member_id,
                member_name: name,
                mobile,
            });
        }
    }

    return Json(members);
}
