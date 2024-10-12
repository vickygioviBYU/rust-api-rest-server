# Overview

My goal is to learn to program in Rust Language, because it is the favorite language of the programmers. It has a lot of interesting functions, like ownership, generic types, and traits. I want to work with powerful libraries and frameworks like Actix.

Rust is a general purpose programming language, is compiled, and is low-memory, what makes it a wonderful language.

This is a API Rest HTTP server developed with Rust and Actix (with MongoDB as a database). This has 5 endpoints:

The courses have three parameters: Code, Name, and Credits.

- POST (We can add courses)
- GET (We can get a course)
- PUT (We can modify a course)
- DELETE (We can delete a course)
- GET (We can get all courses)

In the main function, we initialize the connection to the DB, the collection, creates a new HTTP server, and runs the five endpoints of above. Then bind the server to be listening at 127.0.0.1 in port 8080, and keep running.

I want to develop a server with a database, and to develop some front-end, like GTK, QT, Sciter, etc. Mainly, I wrote this software to learn how to program in Rust. It token a lot of hours and days to get comprehended this language (Rust). If you want to learn it, I recommend that you read a fast tutorial in [www.tutorialspoint.com](www.tutorialspoint.com), then get the Rust Book [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/), learn one time, do not read the advanced part of the tutorial, and learn again once or twice more.

[API REST SERVER with Rust and Actix](https://www.youtube.com/watch?v=3fJWkJ2XCNI)

# Compiling on Windows

The software is tested on Windows. I not guarantee if this software works on Linux, or Mac.
To compile, clone the repository, go to the main folder and run:

```
cargo build
cargo run
```

When the program is executed, takes aprox. 1 minute to load the server. Then you have to open Postman, and throw requests to localhost:8080

# Endpoints and Testing

You can test the software with Postman. Endpoints:

- [POST] /course BODY EXAMPLE: {"code": "CSE 101", "name": "PathwayConect modified", "credits": 3}
- [GET] /course/{id}
- [PUT] /course/{id} BODY EXAMPLE: {"code": "CSE 101", "name": "PathwayConect modified", "credits": 3}
- [DELETE] /course/{id}
- [GET] /courses

# Development Environment

I used:

- VSCode

The most popular multiuse IDE

- Postman

Program to throw HTTP requests, to test APIs

- Rust Lang

Rust Programming Language, a powerful compiled, low-memory usage, with ownership and other interesting things

- Rust Analyzer

VSCode extension to debugging, running, analyzing code, dependency managing, and others

- Actix Web

One of the Rust Lang API REST frameworks

- MongoDB

A document-based database motor, with collections, and can be mounted at MongoDB Atlas, on the cloud

- Chrono

A library to manage time and date

Rust is a general purpose programming language, is compiled, and is low-memory usage, what makes it a wonderful language.

I used:

- Actix Web: One of the Rust Lang API REST frameworks
- MongoDB Rust Driver: Managging connection to MongoDB
- Serde: Utility required by Actix Web
- Dotenv: Library to use environment variables
- Future: Library to allow using asynchronous functions, and ".await"
- Chrono: Library to manage time and date

# Useful Websites

- [Visual Studio Code](https://code.visualstudio.com/)
- [Postman](https://www.postman.com/)
- [Rust Lang](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [MongoDB](https://www.mongodb.com/es)
- [MongoDB Rust Driver](https://www.mongodb.com/docs/drivers/rust/current/)

# Future Work

- Develop a front-end side, with WebAssembly or Desktop
- Add more collections
- Add more columns to the collections
