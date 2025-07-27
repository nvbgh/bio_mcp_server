# Plan for Rust-based MCP Server for Bioinformatics (Test-Driven)

This document outlines a detailed, step-by-step plan to create a Rust-based MCP server. Each step includes a clear action and a corresponding test to verify its success.

---

## Phase 0: Developer Environment & Best Practices

**Goal**: Set up a robust development environment to ensure code quality, consistency, and maintainability.

* [x] #### **Step 0.1: Create a README**

  * **Action**: Create a `README.md` file in the project root. It should briefly describe the project's purpose, how to build and run it, and any other essential information for new contributors.
  * **Test (Manual)**: A `README.md` file exists in the project root and contains the necessary information.

* [x] #### **Step 0.2: Recommend VS Code Extensions**

  * **Action**: Create a `.vscode/extensions.json` file. This file will recommend essential extensions for Rust development to anyone opening the project in VS Code.
        ```json
        {
          "recommendations": [
            "rust-lang.rust-analyzer",
            "tamasfe.even-better-toml",
            "serayuzgur.crates"
          ]
        }
        ```
  * **Test (Manual)**: When the project is opened in VS Code, a notification appears prompting the user to install the recommended extensions.

* [ ] #### **Step 0.3: Configure Code Formatting (rustfmt)**

  * **Action**: `rustfmt` is the standard Rust code formatter and is typically installed with Rust itself. Create a `rustfmt.toml` file in the project root to configure any project-specific formatting rules (even an empty file is fine to signal its use).
  * **Test (Automated)**: Run `cargo fmt -- --check` from the terminal. The command should complete without errors, indicating all files are correctly formatted.

* [ ] #### **Step 0.4: Configure Linting (Clippy)**

  * **Action**: `clippy` is the standard Rust linter. It's an incredibly powerful tool for catching common mistakes and improving code quality. It is typically installed with Rust.
  * **Test (Automated)**: Run `cargo clippy -- -D warnings` from the terminal. This command runs Clippy and treats all warnings as errors. The command should complete without any errors.

* [ ] #### **Step 0.5: Set Up Pre-Commit Hooks**

  * **Action**: Use a tool like `pre-commit` to automate the running of `cargo fmt` and `cargo clippy` before each commit. This ensures that no unformatted code or code with linter warnings is ever committed to the repository.
        1. Install `pre-commit` (e.g., `pip install pre-commit`).
        2. Create a `.pre-commit-config.yaml` file in the root of the project.
        3. Run `pre-commit install` to set up the git hooks.
  * **Test (Manual)**:
        1. Make a small, unformatted change to a Rust file.
        2. Stage the file (`git add .`).
        3. Try to commit the change (`git commit -m "test"`).
        4. **Expected Result**: The commit should fail. The pre-commit hook should run, reformat the file, and then instruct you to re-stage the changes and commit again. The second commit attempt should succeed.

---

## Phase 1: Project Setup & "Hello World" MCP Server

**Goal**: Establish the project structure and confirm that Cursor can launch a basic version of our server.

* [x] #### **Step 1.1: Initialize Rust Project**

  * **Action**: Create a new Rust binary project.

        ```bash
        cargo new rust_mcp_server
        cd rust_mcp_server
        ```

  * **Test (Manual)**: Verify that a `rust_mcp_server` directory is created containing `Cargo.toml` and a `src` directory with `main.rs`.

* [x] #### **Step 1.2: Add Initial Dependencies**

  * **Action**: Edit `Cargo.toml` to include essential libraries for asynchronous operations, serialization, and logging.

        ```toml
        [dependencies]
        tokio = { version = "1", features = ["full"] }
        serde = { version = "1.0", features = ["derive"] }
        serde_json = "1.0"
        tracing = "0.1"
        tracing-subscriber = "0.3"
        ```

  * **Test (Automated)**: Run `cargo check` from the terminal. The command should complete successfully without any errors, confirming the dependencies are correctly specified and can be resolved by Cargo.

* [ ] #### **Step 1.3: Implement a Basic "Echo" Server**

  * **Action**: Modify `src/main.rs` to read a single line from standard input (`stdin`) and print it back to standard output (`stdout`). This simulates receiving and acknowledging a message.
  * **Test (Manual)**:
        1. Run the application with `cargo run`.
        2. The terminal will wait for input. Type a line of text (e.g., `Hello`) and press Enter.
        3. **Expected Result**: The program should immediately print the same line (`Hello`) back to the terminal.

* [ ] #### **Step 1.4: Configure Cursor to Run the Server**

  * **Action**:
        1. Create a new directory `.cursor` in the project's root.
        2. Inside `.cursor`, create a new file named `mcp.json`.
        3. Add the following JSON configuration. **You must replace `"ABSOLUTE_PATH_TO_PROJECT"` with the actual absolute path to the `rust_mcp_server` directory on your machine.**

        ```json
        {
          "mcpServers": {
            "bioinformatics-server": {
              "command": "cargo",
              "args": [
                "run",
                "--quiet",
                "--manifest-path",
                "ABSOLUTE_PATH_TO_PROJECT/Cargo.toml"
              ]
            }
          }
        }
        ```

  * **Test (Manual)**:
        1. Open the `rust_mcp_server` folder in Cursor.
        2. Open the "Chat" panel and look for the list of available tools (you may need to type `@` to see them).
        3. **Expected Result**: You should see `@bioinformatics-server` listed as an available tool. Selecting it won't do anything useful yet, but its presence confirms that Cursor can find and start our server.

---

## Phase 2: GraphQL Client Setup & Mock Query

**Goal**: Set up the `graphql_client` crate and verify it can make a request to a mock server.

* [ ] #### **Step 2.1: Add GraphQL Dependencies**

  * **Action**: Add the `graphql_client` library for making type-safe queries and `reqwest` as the HTTP client. Add `wiremock` for testing.

        ```toml
        # In [dependencies]
        reqwest = { version = "0.11", features = ["json"] }
        graphql_client = "0.13"

        # In [dev-dependencies]
        wiremock = "0.6"
        ```

  * **Test (Automated)**: Run `cargo check`.

* [ ] #### **Step 2.2: Create Placeholder GraphQL Files & Build Script**

  * **Action**:
        1. Create `schema.graphql` in the project root. For now, it can contain a minimal schema: `type Query { gene(name: String!): String }`.
        2. Create `src/queries.graphql` with a sample query: `query GeneQuery($name: String!) { gene(name: $name) }`.
        3. Create `build.rs` in the project root to instruct `graphql_client` to generate Rust code from our schema and queries.
  * **Test (Automated)**: Run `cargo build`. The build should succeed, which proves that the `build.rs` script ran correctly and generated the necessary Rust types from the `.graphql` files.

* [ ] #### **Step 2.3: Implement and Test a Mock GraphQL Request**

  * **Action**: Write a test function that uses `wiremock` to create a mock GraphQL server. The test will then use `reqwest` and the code generated by `graphql_client` to send a request to this mock server and assert that the response is deserialized correctly.
  * **Test (Automated)**: Run `cargo test`. The test should pass, proving our GraphQL client code can correctly construct a query and parse a valid response.

---

## Phase 3: Implement Core MCP Logic

**Goal**: Make the server speak the basic MCP protocol and respond to a tool call with data from the real GraphQL API.

* [ ] #### **Step 3.1: Define MCP Message Structs**

  * **Action**: In a new `src/mcp.rs` file, define the Rust structs that represent the JSON messages for the MCP handshake and tool calls, using `serde::Deserialize` and `serde::Serialize`.
  * **Test (Automated)**: Write a unit test for the `mcp.rs` module. The test should serialize a sample response struct into a JSON string and deserialize a sample request string into a request struct, asserting the contents are correct.

* [ ] #### **Step 3.2: Implement the MCP Handshake**

  * **Action**: Modify `src/main.rs`. When the server starts, it should no longer be a simple echo server. It should now immediately serialize and print an MCP message to `stdout` that defines its available tools (e.g., a single tool named `ask`).
  * **Test (Manual)**:
        1. Open the project in Cursor.
        2. In the chat panel, type `@` to see available tools.
        3. **Expected Result**: The `@bioinformatics-server` tool should now have a sub-item or description showing the `ask` tool we defined.

* [ ] #### **Step 3.3: Handle Tool Call and Connect to Real API**

  * **Action**:
        1. In `src/main.rs`, after the handshake, start a loop to read incoming messages from `stdin`.
        2. If an incoming message is a tool call for `ask`, parse the arguments to get the user's question (e.g., "gene BRCA1").
        3. Call the GraphQL client logic from Phase 2, but this time point it at the **real** GraphQL API endpoint URL (you'll need to provide this).
        4. For now, print the raw JSON response from the GraphQL API to a log file or `stderr` for debugging.
        5. Send back a simple "Work in progress" message to Cursor via `stdout`.
  * **Test (Manual)**:
        1. You will need to provide the real GraphQL API URL and any necessary authentication tokens as environment variables or hardcoded values for this test.
        2. Open the project in Cursor.
        3. In the chat, type `@bioinformatics-server ask "gene BRCA1"`.
        4. **Expected Result**: Cursor chat will show "Work in progress". Check your server's logs or `stderr` output. You should see the full, raw JSON response from your actual bioinformatics GraphQL API. This confirms the end-to-end connection is working.

---

## Phase 4: Formatting and Final Polish

**Goal**: Format the API response into a user-friendly message and handle errors gracefully.

* [ ] #### **Step 4.1: Implement Response Formatting**

  * **Action**: Create a new function that takes the deserialized GraphQL response struct (e.g., a `Gene` struct) and formats it into a human-readable Markdown string.
  * **Test (Automated)**: Write a unit test for the formatting function. Create a sample `Gene` struct instance, pass it to the function, and assert that the output Markdown string is exactly as expected.

* [ ] #### **Step 4.2: Return Formatted Response to Cursor**

  * **Action**: In `src/main.rs`, instead of returning "Work in progress", take the successful GraphQL response, pass it to the new formatting function, and include the resulting Markdown in the MCP tool response sent to `stdout`.
  * **Test (Manual)**:
        1. Open the project in Cursor.
        2. In the chat, type `@bioinformatics-server ask "gene BRCA1"`.
        3. **Expected Result**: The chat window in Cursor should now display a well-formatted Markdown response containing the information about the gene.

* [ ] #### **Step 4.3: Implement Error Handling**

  * **Action**: Wrap the core logic in `main.rs` to handle potential errors (e.g., failed GraphQL request, network issues, invalid user input). If an error occurs, send a properly formatted MCP error response to Cursor.
  * **Test (Manual)**:
        1. **Test 1 (Network Error)**: Disconnect your computer from the internet and run the tool call in Cursor.
        2. **Expected Result 1**: Cursor should display a user-friendly error from the tool, like "I couldn't connect to the bioinformatics API."
        3. **Test 2 (Bad Input)**: Ask a question the server can't parse, like `@bioinformatics-server ask "what is a pizza?"`.
        4. **Expected Result 2**: Cursor should display an error like "I couldn't understand the query. Please ask about a gene, transcript, or mutation."
