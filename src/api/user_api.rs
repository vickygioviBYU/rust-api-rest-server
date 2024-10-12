use crate::{models::user_model::Course, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, get, put, delete, //modify here
    web::{Data, Json, Path}, //modify here
    HttpResponse,
};

use mongodb::bson::oid::ObjectId; //add this

use chrono::prelude::*;

//Post course
#[post("/course")]
pub async fn create_course(db: Data<MongoRepo>, new_course: Json<Course>) -> HttpResponse {
    // New course
    let data = Course {
        id: None,
        code: new_course.code.to_owned(),
        name: new_course.name.to_owned(),
        credits: new_course.credits.to_owned()
    };
    // Call repository to create a course
    let course_detail = db.create_course(data).await;
    match course_detail {
        // Response
        Ok(course) => {
            println!("[{}] Course added", Local::now());
            HttpResponse::Ok().json(course)
        },
        Err(err) => {
            println!("[{}] Error adding course", Local::now());
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

// Get course
#[get("/course/{id}")]
pub async fn get_course(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    // Obtain id
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let course_detail = db.get_course(&id).await;
    match course_detail {
        // Response with course requested
        Ok(course) => {
            println!("[{}] Course fetched", Local::now());
            HttpResponse::Ok().json(course)
        },
        Err(err) => {
            println!("[{}] Error fetching course", Local::now());
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

// Put endpoint
#[put("/course/{id}")]
pub async fn update_course(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_course: Json<Course>,
) -> HttpResponse {
    // Obtain id
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    // New course
    let data = Course {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        code: new_course.code.to_owned(),
        name: new_course.name.to_owned(),
        credits: new_course.credits.to_owned(),
    };
    let update_result = db.update_course(&id, data).await;
    match update_result {
        Ok(update) => {
            println!("[{}] Course modified", Local::now());
            // If one field is modified, get course
            if update.matched_count == 1 {
                let updated_user_info = db.get_course(&id).await;
                return match updated_user_info {
                    // Response with course modified
                    Ok(course) => HttpResponse::Ok().json(course),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => {
            println!("[{}] Error modifying course", Local::now());
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

// Delete endpoint
#[delete("/course/{id}")]
pub async fn delete_course(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    // Obtain id
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    // Call to the repo to delete the course
    let result = db.delete_course(&id).await;
    match result {
        Ok(res) => {
            println!("[{}] Course deleted", Local::now());
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Course successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Course with specified ID not found!");
            }
        }
        Err(err) => {
            println!("[{}] Error deleting course", Local::now());
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

// Get all endpoint
#[get("/courses")]
pub async fn get_all_courses(db: Data<MongoRepo>) -> HttpResponse {
    let courses = db.get_all_courses().await;
    match courses {
        // Call to the repo to return the courses
        Ok(courses) => {
            println!("[{}] Courses fetched", Local::now());
            HttpResponse::Ok().json(courses)
        },
        Err(err) => {
            println!("[{}] Error fetching courses", Local::now());
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}