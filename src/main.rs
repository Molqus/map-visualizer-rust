use map_visualizer::random::draw::draw_data;
use rocket::{
    form::Form,
    fs::{relative, FileServer},
    get, post, routes, FromForm,
};
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
pub struct MapData {
    // #[field(validate = with(|p| p <= &10.0, "CSは10以下で設定してください"))]
    // #[field(validate = with(|p| p >= &0.0, "CSは0以上で設定してください"))]
    cs: f64,
    bpm: f64,
    length: i32,
    index: i32,
}

#[get("/")]
pub fn vis() -> Template {
    let (x, offset_y, fruits_radius) = draw_data(150.0, 0, 4.0, 16);
    let x_reversed: Vec<&f64> = x.iter().rev().collect();
    Template::render(
        "index",
        context! {
            x_list: x_reversed,
            y: offset_y,
            radius: fruits_radius
        },
    )
}

#[post("/", data = "<map_data>")]
pub fn vis_post(map_data: Form<MapData>) -> Template {
    let (bpm, index, cs, length) = (map_data.bpm, map_data.index, map_data.cs, map_data.length);
    let (x, offset_y, fruits_radius) = draw_data(bpm, index, cs, length);
    let x_reversed: Vec<&f64> = x.iter().rev().collect();
    Template::render(
        "index",
        context! {
            x_list: x_reversed,
            y: offset_y,
            radius: fruits_radius,
            bpm: bpm,
            index: index,
            cs: cs,
            length: length
        },
    )
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![vis, vis_post])
        .mount("/assets", FileServer::from(relative!("assets")))
        .attach(Template::fairing());
    Ok(rocket.into())
}

// #[cfg(test)]
// mod test {
//     use super::rocket;
//     use rocket::http::Status;
//     use rocket::local::blocking::Client;
//     use rocket::uri;

//     #[test]
//     fn test_vis() {
//         let client: Client = Client::tracked(rocket()).expect("valid rocket instance");
//         let response = client.get(uri!("/")).dispatch();
//         assert_eq!(response.status(), Status::Ok);
//     }
// }
