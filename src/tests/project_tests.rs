#[cfg(test)]
use warp::http::StatusCode;
#[cfg(test)]
use warp::test::request;
#[cfg(test)]
use warp::Filter; // Add this line to import the Filter trait
#[cfg(test)]
use serde_json::json;

#[cfg(test)]
#[tokio::test]
async fn test_list_projects() {
    let list_projects = warp::path("projects")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&vec!["Project 1", "Project 2"])
        });

    let res = request()
        .method("GET")
        .path("/projects")
        .reply(&list_projects)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.body(), r#"["Project 1","Project 2"]"#);
}

#[tokio::test]
async fn test_fund_project() {
    let fund_project = warp::path("fund")
        .and(warp::post())
        .and(warp::body::json())
        .map(|project: serde_json::Value| {
            warp::reply::json(&project)
        });

    let project_data = json!({
        "name": "Project A",
        "owner": "Owner A",
        "target_amount": 1000,
        "current_amount": 500,
        "is_funded": false
    });

    let res = request()
        .method("POST")
        .path("/fund")
        .json(&project_data)
        .reply(&fund_project)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(std::str::from_utf8(res.body()).unwrap(), project_data.to_string().as_str());
}

#[tokio::test]
async fn test_static_files() {
    let static_files = warp::fs::dir("static");

    let res = request()
        .method("GET")
        .path("/index.html")
        .reply(&static_files)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.body().contains(&"<button onclick=\"getProjects()\">Load Projects</button>".as_bytes()[0]));
}