# Documentation: Building a Web Server with Actix in Rust for Node.js Developers

This tutorial is designed for Node.js developers who are interested in learning how to create a web server using Rust and Actix-web. We'll go through the process step-by-step, comparing the Rust code with equivalent Node.js code to make the transition smoother.

## Table of Contents

- [Introduction](#introduction)
- [Prerequisites](#prerequisites)
- [Project Setup](#project-setup)
- [Adding Dependencies](#adding-dependencies)
- [Writing the Server Code](#writing-the-server-code)
  - [Main Function](#main-function)
  - [Route Handlers](#route-handlers)
- [Understanding Serialization](#understanding-serialization)
  - [Deserialize Trait](#deserialize-trait)
  - [Serialize Trait](#serialize-trait)
- [Middleware](#middleware)
- [Comparing with Node.js Code](#comparing-with-nodejs-code)
- [Running the Server](#running-the-server)
- [Testing the Endpoints](#testing-the-endpoints)
- [Conclusion](#conclusion)
- [Further Reading](#further-reading)

## Introduction

Rust is a systems programming language that emphasizes safety, speed, and concurrency. Actix-web is a high-performance web framework for Rust, enabling developers to create robust web applications. This tutorial is tailored for Node.js developers who are interested in transitioning to Rust, providing a step-by-step guide to creating a web server using Actix-web while comparing it with similar implementations in Node.js.

## Prerequisites

- **Basic knowledge of Rust** syntax and concepts.
- **Rust installed on your system** If not, download it from [rustup.rs](https://rustup.rs/).
- **Familiarity with Node.js and Express.js.**

## Project Setup

To begin, you need to create a new Rust project using Cargo, Rust’s package manager and build system. Open your terminal and run:

```
  cargo new actix-web-server
  cd actix-web-server
```

This command will create a new directory named ```actix-web-server``` with the following structure:

```
  actix-web-server
├── Cargo.toml
└── src
    └── main.rs
```

## Adding Dependencies

Next, you will need to add some dependencies to your project. Open the Cargo.toml file and add the following lines under the [dependencies] section:

```
  [dependencies]
  actix-web = "4"
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  env_logger = "0.9"
```

## Explanation of Dependencies
 - **actix-web**: This is the web framework that will handle HTTP requests and responses.
 - **serde and serde_json**: These libraries are used for serializing and deserializing data structures into and from JSON.
 - **env_logger**: A logging framework that integrates well with Actix’s middleware, helping you log HTTP requests and responses.

## Writing the Server Code

Now, let’s replace the contents of ```src/main.rs``` with the following code:

```
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: String,
}

async fn get_json_data() -> HttpResponse {
    let response = Person {
        name: "Good!".to_string(),
        age: "21".to_string(),
    };

    HttpResponse::Ok().json(response)
}

async fn post_json(info: web::Json<Info>) -> HttpResponse {
    HttpResponse::Ok().json(format!("Received name: {}", info.name))
}

async fn get_user(path: web::Path<(u32,)>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().body(format!("User ID is {}", user_id.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    println!("Starting server at: http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World!") }))
            .route("/users", web::get().to(get_json_data))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(post_json))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

```


## Main Function

The main function is the entry point of your application. Here’s a breakdown of what it does:

 - ```env_logger::init();``` initializes the logging system to log incoming requests.
 - ```HttpServer::new(|| { ... })``` creates a new server instance.
 - ```App::new()``` initializes a new application instance.
 - ```.wrap(Logger::default())``` adds logging middleware to log requests.
 - ```.route(...)``` defines the various routes and their corresponding handlers.
 - ```.bind("127.0.0.1:3000")``` binds the server to the specified address and port.
 - ```.run().await``` starts the server asynchronously.

