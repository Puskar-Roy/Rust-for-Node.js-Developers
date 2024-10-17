# Documentation: Building a Web Server with Actix in Rust for Node.js Developers

This tutorial is designed for Node.js developers who are interested in learning how to create a web server using Rust and Actix-web. We'll go through the process step-by-step, comparing the Rust code with equivalent Node.js code to make the transition smoother.

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


## Route Handlers

In this section, you will define various route handlers that respond to specific HTTP requests:

 - GET /users: Returns a JSON representation of a person.
 - POST /users: Accepts JSON data and returns a confirmation message.
 - GET /users/{id}: Returns the user ID based on the path parameter.

## Understanding Serialization

Serialization is the process of converting a data structure into a format that can be easily stored or transmitted. In Rust, you can use the Serde library to handle this process.

## Deserialize Trait

The Deserialize trait allows Rust to convert JSON data into Rust structs. In the code, we defined a struct Info that will hold the name passed in a JSON payload.

## Serialize Trait

The Serialize trait enables Rust to convert Rust structs into JSON format. In the code, the Person struct is defined to hold a name and age, which can be converted to JSON when sending a response.

## Middleware

Middleware in Actix-web provides a way to execute code before or after handling requests. In this example, the Logger middleware is used to log incoming HTTP requests, helping with debugging and monitoring.

## Comparing with Node.js Code

For those familiar with Node.js and Express.js, here’s how a similar server would look:

```
const express = require('express');
const app = express();
app.use(express.json());

app.get('/users', (req, res) => {
    res.json({ name: 'Good!', age: '21' });
});

app.post('/users', (req, res) => {
    res.json(`Received name: ${req.body.name}`);
});

app.get('/users/:id', (req, res) => {
    res.send(`User ID is ${req.params.id}`);
});

app.listen(3000, () => {
    console.log('Server is running on http://localhost:3000');
});

```

## Running the Server

To run the server, execute the following command in your terminal:

```
  cargo run
```

This command will start the Actix-web server at ``http://localhost:3000``.


## Testing the Endpoints

You can test your endpoints using tools like Postman or cURL. Here are some examples:

- GET /users:

```
curl http://localhost:3000/users
```
- POST /users:

```
curl -X POST -H "Content-Type: application/json" -d '{"name": "John"}' http://localhost:3000/users
```
- GET /users/{id}:

```
curl http://localhost:3000/users/1
```


## Conclusion

Congratulations! You’ve successfully built a web server using Rust and Actix-web, comparable to a Node.js/Express.js server. This experience should provide a solid foundation for exploring further Rust web development.
