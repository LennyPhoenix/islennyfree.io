mod schedule;

use axum::{response::Html, routing::get, Router};
use chrono::{NaiveTime, Utc};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;

async fn app() -> Html<String> {
    let activities = schedule::load_schedule("activities");

    let now = Utc::now();

    let colour: &str;
    let html = if let Some(activity) = schedule::current_activity(&now, &activities) {
        let next_free = schedule::next_free(&now, &activities);
        let remaining = (next_free - now).num_minutes();

        let time = activity
            .active_timeframe(&now)
            .unwrap()
            .end()
            .format("%H:%M:%S");

        colour = "#ff3a3a";

        rsx! {
            h1 { "No :(" },
            h2 { "Lenny is currently busy with {activity.name()} until {time}." },
            h3 { "(He will next be free in {remaining} minutes...)" },
        }
    } else {
        let next = activities
            .iter()
            .map(|a| a.times())
            .flatten()
            // Activity is today
            .filter(|tf| tf.today(&now))
            // Start is after now
            .filter(|tf| tf.start() > &now.time())
            // Get earliest
            .min_by_key(|tf| tf.start());

        let sub = if let Some(next) = next {
            let remaining = (*next.start() - now.time()).num_minutes();
            let time = next.start().format("%H:%M:%S");
            rsx! {
                h2 { "Lenny is currently free until {time}." },
                h3 { "(He will be free for another {remaining} minutes...)"},
            }
        } else {
            rsx! {
                h2 { "Lenny has no more activities to do today! Hooray :)" }
            }
        };

        colour = "#1bff5e";

        rsx! {
            h1 { "Yes!" },
            sub
        }
    };

    Html(render_lazy(rsx! {
        body {
            style: "background-color: {colour}; color: white;",
            div {
                style: "align-items: center; display: flex; height: 100vh; justify-content: center; flex-direction: column;",
                html
            }
        }
    }))
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}!");

    axum::Server::bind(&addr)
        .serve(Router::new().route("/", get(app)).into_make_service())
        .await
        .unwrap();
}
