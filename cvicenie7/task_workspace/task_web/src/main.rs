use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, ResponseError, delete, get, put, web};
use chrono::{NaiveDate, TimeDelta};
use serde::{Deserialize, Serialize};
use task_library::control;
use task_library::control::db::create_from_db;
use task_library::task::ReadTaskFromUser;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(hello, get_tasks, get_task_by_id, add_task, delete_task),
    components(schemas(SimpleResponse))
)]
struct ApiDoc;

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleResponse {
    status: u16,
}

// struct AppState {
//     task_manager: TaskManager,
// }

#[utoipa::path(
    responses(
        (status = 200, description = "API is alive", body = SimpleResponse)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse { status: 200 })
}

#[utoipa::path]
#[get("/get_tasks")]
async fn get_tasks() -> impl Responder {
    let result = web::block(create_from_db).await;

    match result {
        Ok(manager) => HttpResponse::Ok().json(manager.get_tasks()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path]
#[get("/get_task_by_id/{task_id}")]
async fn get_task_by_id(path: web::Path<usize>) -> impl Responder {
    let task_id = path.into_inner();

    let result = web::block(move || control::db::show_task_by_id(task_id)).await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::NotFound().body("Task not found"),
    }
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
struct AddTaskData {
    pub id: usize,
    pub nazov: String,
    pub popis: String,
    pub priorita: i32,
    pub planovany_zaciatok: String,
    pub planovane_trvanie: i64,
    pub skutocny_zaciatok: Option<String>,
    pub skutocne_trvanie: Option<i64>,
}

impl ReadTaskFromUser for AddTaskData {
    fn read_id(&self, _msg: &str) -> usize {
        self.id
    }

    fn read_nazov(&self, _msg: &str) -> String {
        self.nazov.clone()
    }

    fn read_popis(&self, _msg: &str) -> String {
        self.popis.clone()
    }

    fn read_priorita(&self, _msg: &str) -> i32 {
        self.priorita
    }

    fn read_planovany_zaciatok(&self, _msg: &str) -> NaiveDate {
        NaiveDate::parse_from_str(&self.planovany_zaciatok, "%Y-%m-%d")
            .expect("Invalid date format, expected YYYY-MM-DD")
    }

    fn read_planovane_trvanie(&self, _msg: &str) -> TimeDelta {
        TimeDelta::minutes(self.planovane_trvanie)
    }

    fn read_skutocny_zaciatok(&self, _msg: &str) -> Option<NaiveDate> {
        self.skutocny_zaciatok
            .as_ref()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").expect("Invalid date format"))
    }

    fn read_skutocne_trvanie(&self, _msg: &str) -> Option<TimeDelta> {
        self.skutocne_trvanie.map(TimeDelta::minutes)
    }
}

#[utoipa::path]
#[put("/add_task")]
async fn add_task(data: web::Json<AddTaskData>) -> impl Responder {
    let payload = data.into_inner();

    let result = web::block(move || {
        control::db::add_task(&payload);
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Task added"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path]
#[delete("/delete_task/{id}")]
async fn delete_task(path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();

    let result = web::block(move || control::db::remove_task_by_id(id)).await;

    match result {
        Ok(true) => HttpResponse::Ok().json("Deleted"),
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let openapi = ApiDoc::openapi();
    // let app_state = web::Data::new(Mutex::new(AppState {
    //     task_manager: TaskManager::new(),
    // }));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                Files::new("/app", "./static")
                    .index_file("index.html")
                    .show_files_listing(),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
            .service(get_tasks)
            .service(get_task_by_id)
            .service(add_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
