use gloo_dialogs::prompt;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// Task structure
#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct Task {
    id: usize,
    nazov: String,
    popis: String,
    priorita: i32,
    planovany_zaciatok: String,
    planovane_trvanie: i64,
}

#[function_component(App)]
fn app() -> Html {
    let tasks = use_state(|| vec![]);

    // Fetch tasks
    {
        let tasks = tasks.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                // fetch tasks
                let fetched: Vec<Task> = Request::get("http://127.0.0.1:8081/get_tasks")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                tasks.set(fetched);
            });
            || ()
        });
    }

    // Add task
    let add_task = {
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let name = prompt("Task name:", None).unwrap_or_default();
                if name.is_empty() {
                    return;
                }

                let new_task = Task {
                    id: 0,
                    nazov: name.clone(),
                    popis: "Desc".into(),
                    priorita: 1,
                    planovany_zaciatok: "2025-12-01".into(),
                    planovane_trvanie: 60,
                };

                let _res = Request::put("http://127.0.0.1:8081/add_task")
                    .json(&new_task)
                    .unwrap()
                    .send()
                    .await;

                // Refresh tasks
                let fetched: Vec<Task> = Request::get("http://127.0.0.1:8081/get_tasks")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                tasks.set(fetched);
            });
        })
    };

    // Delete task
    let delete_task = {
        let tasks = tasks.clone();
        Callback::from(move |id: usize| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let _res = Request::delete(&format!("http://127.0.0.1:8081/delete_task/{}", id))
                    .send()
                    .await;

                // Refresh tasks
                let fetched: Vec<Task> = Request::get("http://127.0.0.1:8081/get_tasks")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                tasks.set(fetched);
            });
        })
    };

    html! {
        <div class="container">
            <h1>{ "Task Manager" }</h1>
            <button class="add-btn" onclick={add_task}>{ "Add Task" }</button>
            <ul>
                { for tasks.iter().map(|task| {
                    let id = task.id;
                    let delete_cb = {
                        let delete_task = delete_task.clone();
                        Callback::from(move |_| delete_task.emit(id))
                    };
                    html! {
                        <li key={task.id} class="task-item">
                            { format!("{} - {}", task.id, task.nazov) }
                            <button class="delete-btn" onclick={delete_cb}>{ "Delete" }</button>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}

// Entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
