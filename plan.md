# Plan for Rust MCP Server: Proof of Concept First

This plan is structured to deliver a working Proof of Concept (POC) as quickly as possible. The first goal is to make the server correctly communicate with the Cursor chat using the MCP protocol. Complex features like GraphQL will be added *after* the core is proven to work.

---

## Phase 0: Environment Setup (Completed)
* [x] All setup tasks are complete.

---

## Phase 1: Basic Command Runner (Completed)
* [x] The project is set up.
* [x] The server runs and can respond to simple text commands (`ping`, `date`).
* [x] Cursor is configured to start the server.

---

## **Phase 2: Implement MCP Protocol (Current Focus)**

**Goal**: Achieve a working end-to-end POC by teaching the server to speak MCP.

* [x] #### **Step 2.1: Define MCP Message Structs**

  * **Action**: In a new `src/mcp.rs` file, define the Rust structs for MCP handshake and tool call messages using `serde`.
  * **Test (Automated)**: Create a unit test to verify that serializing and deserializing the structs works correctly.

* [x] #### **Step 2.2: Implement MCP Handshake**

  * **Action**: On startup, make the server send a valid MCP handshake message to `stdout`. This message will advertise the `ping` and `date` tools.
  * **Test (Manual)**: In Cursor, typing `@` should show `@bioinformatics-server` with `ping` and `date` as available sub-tools.

* [ ] #### **Step 2.3: Handle MCP Tool Calls & Respond**

  * **Action**: Update the main loop to parse incoming MCP tool call messages from `stdin`. When a call for `ping` or `date` is received, run the existing logic and send back the result in a valid MCP response message to `stdout`.
  * **Test (Manual)**: Running `@bioinformatics-server ping` in chat should result in `pong` being displayed.

---

## Phase 3: Add Real Functionality (GraphQL)

**Goal**: Replace a demo tool with a real query to the bioinformatics API.

* [ ] #### **Step 3.1: Add GraphQL Dependencies & Files**
    * **Action**: Add `graphql_client`, `reqwest`, and `wiremock`. Create `schema.graphql`, `queries.graphql`, and `build.rs`.
    * **Test**: `cargo build` should succeed.

* [ ] #### **Step 3.2: Implement Mock GraphQL Client**
    * **Action**: Write a test using `wiremock` to confirm the GraphQL client can make a request and parse a response.
    * **Test**: `cargo test` for the mock client should pass.

* [ ] #### **Step 3.3: Connect GraphQL to a Tool**
    * **Action**: Create a new tool (e.g., `ask_gene`). When called, it should use the GraphQL client to query the real API.
    * **Test**: `@bioinformatics-server ask_gene "BRCA1"` should result in the raw API JSON appearing in server logs.

---

## Phase 4: Polish

**Goal**: Improve user experience and error handling.

* [ ] #### **Step 4.1: Format API Response**
* [ ] #### **Step 4.2: Add Error Handling**
