# Project: Unidoku

## Project Overview

This project is a Sudoku game named "Unidoku," developed as a Progressive Web App (PWA). The core logic is written in Rust and compiled to WebAssembly (WASM), making it a high-performance web application. The frontend is built with standard HTML, CSS, and JavaScript, which acts as a loader and interface for the WASM module.

The application uses `wasm-pack` to build the Rust code into a web-compatible package. It is set up for continuous deployment to GitHub Pages using a GitHub Actions workflow.

**Key Technologies:**
- **Rust:** For the core game logic.
- **WebAssembly (WASM):** To run the Rust code in the browser.
- **`wasm-bindgen`:** To facilitate interoperability between Rust/WASM and JavaScript.
- **`web-sys`:** To interact with browser APIs (like the DOM) directly from Rust.
- **HTML/CSS/JS:** For the application's structure, styling, and WASM initialization.
- **GitHub Actions:** For CI/CD and automated deployment to GitHub Pages.

## Building and Running

### Building the Project

The project is built using `wasm-pack`. The following command compiles the Rust code into a WebAssembly package and generates the necessary JavaScript bindings.

```bash
# Build for the web target
wasm-pack build --target web
```
This command creates a `pkg` directory containing the WASM file, a JavaScript wrapper, a TypeScript definition file, and a `package.json`.

### Running Locally

To run the application locally, you need to serve the root directory with a simple HTTP server after building the project.

1.  **Build the project:**
    ```bash
    wasm-pack build --target web
    ```

2.  **Start a local server:**
    You can use any simple static file server. A common choice is Python's built-in server.

    ```bash
    # From the project root directory
    python -m http.server
    ```
    
    Then, open your browser to `http://localhost:8000`.

## Development Conventions

- **DOM Manipulation:** All DOM manipulation (creating the board, handling input) is performed within the Rust code using the `web-sys` crate. JavaScript's role is limited to loading the WASM module and registering the service worker.
- **State Management:** The entire game state, including the board and user input, is managed within the `Sudoku` struct in Rust.
- **Event Handling:** User interactions (typing in cells, clicking buttons) are handled by closures in Rust that are attached as event listeners to the DOM elements.
- **Deployment:** The project is automatically deployed to the `gh-pages` branch from the `main` branch. The workflow builds all artifacts into a `dist` directory, which is then pushed to the `gh-pages` branch for hosting.
