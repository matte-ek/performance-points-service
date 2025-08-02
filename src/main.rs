use axum::http::StatusCode;
use axum::routing::get;
use axum::{
    extract::Json,
    Router,
};
use rosu_pp::model::mods::rosu_mods::GameModsLegacy;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PpMapRequest {
    map_id: u64,
    mods: u32,
}

#[derive(Serialize)]
struct PpMapResponse {
    pp100: f64,
    pp98: f64,
    pp95: f64,
}

#[derive(Deserialize)]
struct PpScoreRequest {
    map_id: u64,
    mods: u32,
    n300: u32,
    n100: u32,
    n50: u32,
    miss: u32,
    max_combo: u32,
}

#[derive(Serialize)]
struct PpScoreResponse {
    pp: f64,
    pp_max: f64
}

async fn calculate_map_pp(Json(req): Json<PpMapRequest>) -> Result<(StatusCode, Json<PpMapResponse>), StatusCode> {
    let map_path = format!("./cache/{}.osu", req.map_id);

    let map = match rosu_pp::Beatmap::from_path(map_path) {
        Ok(map) => map,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND)
        }
    };

    if let Err(_) = map.check_suspicion() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mods = GameModsLegacy::from_bits(req.mods);

    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(mods)
        .calculate(&map);

    let pp100 = rosu_pp::Performance::new(diff_attrs.clone())
        .lazer(false)
        .mods(mods)
        .accuracy(100.0)
        .calculate()
        .pp();

    let pp98 = rosu_pp::Performance::new(diff_attrs.clone())
        .lazer(false)
        .mods(mods)
        .accuracy(98.0)
        .calculate()
        .pp();

    let pp95 = rosu_pp::Performance::new(diff_attrs.clone())
        .lazer(false)
        .mods(mods)
        .accuracy(95.0)
        .calculate()
        .pp();

    Ok((
        StatusCode::OK,
        Json(PpMapResponse {
            pp100,
            pp98,
            pp95
        })
    ))
}

async fn calculate_score_pp(Json(req): Json<PpScoreRequest>) -> Result<(StatusCode, Json<PpScoreResponse>), StatusCode> {
    let map_path = format!("./cache/{}.osu", req.map_id);

    let map = match rosu_pp::Beatmap::from_path(map_path) {
        Ok(map) => map,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND)
        }
    };

    if let Err(_) = map.check_suspicion() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let mods = GameModsLegacy::from_bits(req.mods);

    let diff_attrs = rosu_pp::Difficulty::new()
        .mods(mods)
        .calculate(&map);

    let pp = rosu_pp::Performance::new(diff_attrs.clone())
        .lazer(false)
        .n300(req.n300)
        .n100(req.n100)
        .n50(req.n50)
        .misses(req.miss)
        .combo(req.max_combo)
        .mods(mods)
        .calculate()
        .pp();

    let pp_max = rosu_pp::Performance::new(diff_attrs.clone())
        .lazer(false)
        .n300(req.n300)
        .n100(req.n100)
        .n50(req.n50)
        .misses(0)
        .mods(mods)
        .calculate()
        .pp();

    Ok((
        StatusCode::OK,
        Json(PpScoreResponse {
            pp,
            pp_max
        })
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pp/score", get(calculate_score_pp))
        .route("/pp/map", get(calculate_map_pp));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}