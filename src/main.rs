mod project;

use crate::project::Project;
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Serialize)]
struct TransferRequest {
    project_name: String,
}

#[derive(Deserialize, Serialize)]
struct CreateProjectRequest {
    name: String,
    owner: String,
    target_amount: u64,
}

#[tokio::main]
async fn main() {
    let projects = Arc::new(Mutex::new(vec![
        Project::new("Project 1", "Owner 1", 1000),
        Project::new("Project 2", "Owner 2", 2000),
    ]));

    let list_projects = warp::path("projects")
        .and(warp::get())
        .map({
            let projects = Arc::clone(&projects);
            move || {
                let projects = projects.lock().unwrap();
                warp::reply::json(&*projects)
            }
        });

    let create_project = warp::path("projects")
        .and(warp::post())
        .and(warp::body::json())
        .map({
            let projects = Arc::clone(&projects);
            move |create_request: CreateProjectRequest| {
                let mut projects = projects.lock().unwrap();
                let new_project = Project::new(
                    create_request.name.as_str(),
                    create_request.owner.as_str(),
                    create_request.target_amount,
                );
                projects.push(new_project);
                warp::reply::json(&"Project created")
            }
        });

    let transfer_funds = warp::path("transfer")
        .and(warp::post())
        .and(warp::body::json())
        .map({
            let projects = Arc::clone(&projects);
            move |transfer_request: TransferRequest| {
                let mut projects = projects.lock().unwrap();
                if let Some(project) = projects.iter_mut().find(|p| p.name == transfer_request.project_name) {
                    project.transfer_funds();
                    warp::reply::json(&format!("Funds transferred for project: {}", project.name))
                } else {
                    warp::reply::json(&format!("Project not found: {}", transfer_request.project_name))
                }
            }
        });

    let static_files = warp::fs::dir("static");

    let routes = list_projects.or(create_project).or(transfer_funds).or(static_files);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod tests;