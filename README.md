# RustWave


# Web Server in Rust (RustWave) - README

This project is a simple implementation of a web server in Rust. The server is designed to serve static files from a specified public directory and responds to HTTP requests with the appropriate content or error messages.

**How to Use:**
1. Clone the repository to your local machine.
2. Ensure you have Rust and Cargo installed.
3. Navigate to the project directory in the terminal.
4. Modify the `PUBLIC_PATH` environment variable in the `main` function of `main.rs` to specify the public directory where your static files are located. Alternatively, it will use the `public` directory in the project root by default.
5. Run the server by executing the following command:
6. The server will start listening on `127.0.0.1:8080` by default. You can change the address in the `Server::new()` call in the `main` function of `main.rs`.
7. Access the server using your web browser or any HTTP client, and it will serve the static files located in the specified public path.


## Instructions to Run

1. Clone the repository to your local machine.
2. Ensure you have Rust and Cargo installed.
3. Navigate to the project directory in the terminal.
4. Modify the `PUBLIC_PATH` environment variable in the `main` function of `main.rs` to specify the public directory where your static files are located. Alternatively, it will use the `public` directory in the project root by default.
5. Run the server by executing the following command:



**Customization:**
- To extend the functionality, implement the `Handler` trait with a custom handler for your specific application logic.
- You can add more HTTP methods in the `Method` enum if needed.
- Enhance error handling and logging as per your requirements.
- For production deployment, consider adding security features, load balancing, and more robust error handling.


**Features:**
- HTTP Handling: The code includes structures and enums for handling HTTP requests and responses.
- Handler Trait: A `Handler` trait is defined, allowing custom request handlers to be implemented.
- Server Struct: The `Server` struct represents the web server and listens for incoming connections on the specified address.
- WebsiteHandler Struct: The `WebsiteHandler` struct implements the `Handler` trait and serves static files from a designated public path.
- Parsing HTTP Requests: The code parses incoming HTTP requests to extract the method, path, and protocol.
- Main Function: The main function sets up and runs the server with the `WebsiteHandler`.


## Features

- HTTP Handling: The code includes structures and enums for handling HTTP requests and responses.
- Handler Trait: A `Handler` trait is defined, allowing custom request handlers to be implemented.
- Server Struct: The `Server` struct represents the web server and listens for incoming connections on the specified address.
- WebsiteHandler Struct: The `WebsiteHandler` struct implements the `Handler` trait and serves static files from a designated public path.
- Parsing HTTP Requests: The code parses incoming HTTP requests to extract the method, path, and protocol.
- Main Function: The main function sets up and runs the server with the `WebsiteHandler`.

## Future Enhancements

- **Multi-threading:** Implement multi-threading to allow the server to handle multiple connections simultaneously, improving concurrency and throughput. One approach is to use the `std::thread` module to spawn threads for each incoming connection. Be mindful of synchronization and potential race conditions when sharing data between threads.

- **Async/Await:** Introduce asynchronous features to enhance the server's efficiency. Rust has excellent support for asynchronous programming using the `async` and `await` keywords. You can use libraries like `tokio` or `async-std` to create an asynchronous web server. Asynchronous I/O allows the server to handle multiple connections without creating additional threads for each connection.

- **Connection Pooling:** Implement connection pooling to manage database connections efficiently. This can help reduce connection overhead and improve the server's responsiveness.

- **Dynamic Routing:** Enhance the request handling by introducing dynamic routing. Instead of hardcoding paths, implement a routing system that can handle different endpoints and route requests to appropriate handlers.

- **Middleware:** Implement middleware to add additional functionality to the request/response flow. Middleware can handle tasks like logging, authentication, compression, etc., before or after the main request handler.

- **TLS/SSL Support:** Add support for secure connections using TLS/SSL certificates to encrypt data exchanged between clients and the server.

- **Error Handling and Logging:** Enhance error handling with detailed error messages and implement a logging system to track server activities and monitor potential issues.

- **Load Balancing:** If you plan to deploy the server in a distributed environment, consider implementing load balancing to distribute incoming requests among multiple server instances.

- **Configuration Management:** Implement a configuration system to allow users to customize server settings, such as the port number, public directory, or any other server-specific configurations.

- **Unit Testing and Integration Testing:** Write comprehensive unit tests and integration tests to ensure the stability and correctness of the server code.


**Disclaimer:**
This project is intended for educational and basic use purposes. For production applications, it is recommended to use more mature and feature-rich web server libraries available in the Rust ecosystem.

**Contributing:**
Contributions to this project are welcome. If you find any issues or have suggestions for improvements, feel free to open a pull request or create an issue on the project's GitHub repository. Happy coding!
