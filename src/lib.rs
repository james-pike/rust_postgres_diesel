#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use models::{NewPost, Post};
use std::io::{stdin, Read};


pub struct DieselDemo {
    database_connection: PgConnection,
}

impl DieselDemo {
    pub fn new(database_url: String) -> DieselDemo {
        let database_connection =
            PgConnection::establish(&database_url).expect("error connecting to the database");
        DieselDemo {
            database_connection,
        }
    }

    pub fn run(&self) {

        // TODO ask user if we want to read all posts
        self.display_all_posts();
        // TODO ask the user if we want to add a post
        //self.add_new_post();
        // TODO ask the user if we want to publish a post
        //self.publish_post();
        // TODO ask the user if we want to delete a post
        //self.delete_post();
    }

    fn display_all_posts(&self) {
        use schema::posts::dsl::*;

        let all_posts = posts
            .filter(published.eq(true))
            .load::<Post>(&self.database_connection)
            .expect("Error getting all posts");
        println!("Displaying all posts\n");
        for post in all_posts {
            println!("{}", post.title);
            println!("-----------------");
            println!("{}", post.body);
            println!("");
        }
    }

    fn add_new_post(&self) {
        use schema::posts;
        println!("Creating new post");
        println!("Title: ");
        let mut title = String::new();
        stdin().read_line(&mut title).unwrap();

        println!("Body:");
        let mut body = String::new();
        stdin().read_line(&mut body).unwrap();
        let new_post = NewPost::new(title, body);
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<Post>(&self.database_connection)
            .expect("error adding post");
    }

    fn publish_post(&self) {
        use schema::posts::dsl::*;
        self.display_unpublished_titles();
        println!("What post do you want to publish");
        let mut post_id = String::new();
        stdin().read_line(&mut post_id).unwrap();
        let post_id = post_id.trim().parse::<i32>().unwrap();
        diesel::update(posts.find(post_id))
            .set(published.eq(true))
            .execute(&self.database_connection)
            .expect("Error publishing post");
    }

    fn display_unpublished_titles(&self) {
        use schema::posts::dsl::*;

        let all_posts = posts
            .filter(published.eq(false))
            .load::<Post>(&self.database_connection)
            .expect("Error getting all unplublished posts");
        println!("Displaying all post titles");
        for post in all_posts {
            println!("{}: {}", post.id, post.title.trim());
        }
    }

    fn display_all_titles(&self) {
        use schema::posts::dsl::*;

        let all_posts = posts
            .load::<Post>(&self.database_connection)
            .expect("Error getting all unplublished posts");
        println!("Displaying all post titles");
        for post in all_posts {
            println!("{}: {}", post.id, post.title.trim());
        }
    }

    fn delete_post(&self) {
        use schema::posts::dsl::*;
        self.display_all_titles();
        println!("What post do you want to delete");
        let mut post_id = String::new();
        stdin().read_line(&mut post_id).unwrap();
        let post_id = post_id.trim().parse::<i32>().unwrap();
        diesel::delete(posts.find(post_id))
            .execute(&self.database_connection)
            .expect("Error deleting post");
    }
}