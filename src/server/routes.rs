use actix_web::{get, patch, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use super::encryption::{hash_password, verify_password};

#[derive(Serialize, Deserialize)]
struct User {
    gamer_id: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Leaderboard {
    gamer_id: String,
    high_score: i32,
    time: String,
}

#[get("/")]
pub async fn health_check() -> impl Responder {
    "OK"
}

#[post("/user/login")]
pub async fn get_or_add_user(
    pool: web::Data<PgPool>, 
    user_credentials: web::Json<User>
) -> impl Responder {
    let gamer_id = &user_credentials.gamer_id;
    let password = &user_credentials.password;

    let hashed_password = hash_password(password).unwrap();

    // Query the database to check if the user exists
    let result = sqlx::query_as!(
        User, 
        "SELECT gamer_id, password FROM users WHERE gamer_id = $1", 
        gamer_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(users) => {
            if users.is_empty() {
                // If the user doesn't exist, insert them
                let result = sqlx::query_as!(
                    User, 
                    "INSERT INTO users (gamer_id, password) VALUES ($1, $2) RETURNING gamer_id, password", 
                    gamer_id, hashed_password
                )
                .fetch_all(pool.get_ref())
                .await;
                match result {
                    Ok(users) => HttpResponse::Ok().json(users),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            } else if verify_password(&password, &users[0].password) == true {
                // If everything is correct, return the user data
                HttpResponse::Ok().json(users)
            } else {
                // The provided password doesn't match, return Unauthorized
                HttpResponse::Unauthorized().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/leaderboard")]
pub async fn get_leaderboard(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(Leaderboard, "SELECT gamer_id, high_score, time FROM high_score ORDER BY high_score DESC LIMIT 5;")
        .fetch_all(pool.get_ref())
        .await;

    match result {    
        Ok(leaderboard) => HttpResponse::Ok().json(leaderboard),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/stats/{gamer_id}")]
pub async fn get_stats(pool: web::Data<PgPool>, params: web::Path<String>) -> impl Responder {
    let gamer_id = params.into_inner();
    let result = sqlx::query_as!(Leaderboard, "SELECT gamer_id, high_score, time FROM high_score WHERE gamer_id = $1", gamer_id)
        .fetch_all(pool.get_ref())
        .await;

    match result {    
        Ok(leaderboard) => HttpResponse::Ok().json(leaderboard),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/update_stats")]
pub async fn update_stats(pool: web::Data<PgPool>, game_stats: web::Json<Leaderboard>) -> impl Responder {
    let gamer_id = &game_stats.gamer_id;
    let high_score = &game_stats.high_score;
    let time = &game_stats.time;

    let result = sqlx::query!(
        r#"
            INSERT INTO high_score (gamer_id, high_score, time)
            VALUES ($1, $2, $3)
            ON CONFLICT (gamer_id)
            DO UPDATE 
            SET high_score = GREATEST(high_score.high_score, EXCLUDED.high_score),
                time = CASE WHEN high_score.high_score < EXCLUDED.high_score THEN EXCLUDED.time ELSE high_score.time END
            RETURNING gamer_id, high_score, time;
        "#,
        gamer_id,
        high_score,
        time
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(row) => HttpResponse::Ok().json(Leaderboard {
            gamer_id: row.gamer_id,
            high_score: row.high_score,
            time: row.time,

        }),
        Err(e) => {
            println!("Error updating stats");
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
