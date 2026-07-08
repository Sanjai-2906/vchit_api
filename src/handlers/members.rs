use axum::{Json, extract::{Path, State}};
use oracle::Connection;

use crate::{AppConfig, models::MemberModel};
pub async fn get_members(
    State(config): State<AppConfig>,
    grp_name: Option<Path<String>>,
) -> Json<Vec<MemberModel>> {
    let conn = Connection::connect(
        &config.oracle_user,
        &config.oracle_password,
        &config.oracle_connect_string,
    )
    .unwrap();
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
