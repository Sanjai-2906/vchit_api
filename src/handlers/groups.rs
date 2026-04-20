use axum::Json;

pub async fn get_groups() -> Json<Vec<String>> {
    let group_list: Vec<&str> = vec![
        "1L A1", "1L A2", "1L A3", "2L A1", "2L A2", "2L A3", "3L A1", "3L A2", "3L A3", "4L A1",
        "4L A2", "4L A3", "5L A1", "5L A2", "5L A3",
    ];

    let groups: Vec<String> = group_list.iter().map(|group|group.to_string()).collect();

    return Json(groups);
}
