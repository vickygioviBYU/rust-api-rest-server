use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult}, //modify here
    Client, Collection,
};

use futures::stream::TryStreamExt; //add this

use crate::models::user_model::Course;

// Struct representing the MongoDB Repository, with the collection of courses
pub struct MongoRepo {
    col: Collection<Course>,
}

// Implementation of MongoRepo
impl MongoRepo {
    // Init repo
    pub async fn init() -> Self {
        // Init environment variables
        dotenv().ok();
        // Obtain database URI
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        // Obtain client handler object
        let client = Client::with_uri_str(uri).await.unwrap();
        // Obtain database handler object
        let db = client.database("rustDB");
        // Obtain collection of courses
        let col: Collection<Course> = db.collection("Course");
        // Return structure
        MongoRepo { col }
    }

    /*
    pub struct Course {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub code: String,
    pub name: String,
    pub credits: u8,
    }
     */

    pub async fn create_course(&self, new_course: Course) -> Result<InsertOneResult, Error> {
        let new_doc = Course {
            id: None,
            code: new_course.code,
            name: new_course.name,
            credits: new_course.credits,
        };

        // Call collection, insert a new document, await the response of the DB, handle errors, and return the result of the insertion
        let course = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating course");
        Ok(course)
    }

    pub async fn get_course(&self, id: &String) -> Result<Course, Error> {
        // Obtain object id
        let obj_id = ObjectId::parse_str(id).unwrap();
        // Filter by id
        let filter = doc! {"_id": obj_id};
        // Run find one query on MongoDB, pass the filter, await response, handle errors, and return the course
        let course_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting course's detail");
        Ok(course_detail.unwrap())
    }

    pub async fn update_course(&self, id: &String, new_course: Course) -> Result<UpdateResult, Error> {
        // Obtain object id
        let obj_id = ObjectId::parse_str(id).unwrap();
        // Filter by ID
        let filter = doc! {"_id": obj_id};
        // New document to replace the course
        let new_doc = doc! {
            "$set":
                {
                    "code": new_course.code,
                    "name": new_course.name,
                    "credits": new_course.credits
                },
        };
        // Run query update one, await, handle errors, and return update result
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_course(&self, id: &String) -> Result<DeleteResult, Error> {
        // Obtain id
        let obj_id = ObjectId::parse_str(id).unwrap();
        // Filter by id
        let filter = doc! {"_id": obj_id};
        // Run query delete one, pass filter, await, handle errors, and return delete result
        let course_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(course_detail)
    }

    pub async fn get_all_courses(&self) -> Result<Vec<Course>, Error> {
        // Find all the elements of the collection, without filter, await, handle errors
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of courses");
        // Vector to returning all courses
        let mut courses: Vec<Course> = Vec::new();
        // Iterate in cursor
        while let Some(course) = cursors
            // Iterate
            .try_next()
            // Await response
            .await
            .ok()
            // Handle errors
            .expect("Error mapping through cursor")
        {
            // Push all the courses at the vector
            courses.push(course)
        }
        // Return courses
        Ok(courses)
    }
}

