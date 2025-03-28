# Breakout Arcade Game - Backend (Rust + PostgreSQL)
Repo to hold the Server &amp; DB code for breakout-game.<br>
Link to the front-end repo: https://github.com/tanrode/breakout-game

# Dependencies
<ul>
  <li><b>actix-web</b>: A lightweight web framework for building scalable web applications and APIs in Rust.</li>
  <li><b>bcrypt</b>: Used for hashing and verifying passwords securely.</li>
  <li><b>dotenvy</b>: Loads environment variables from a `.env` file for easy configuration management.</li>
  <li><b>sqlx</b>: An async SQL database client supporting various databases with query validation.</li>
  <li><b>serde</b>: Facilitates serialization and deserialization of data using Rust structs.</li>
  <li><b>tokio</b>: Provides a set of utilities for asynchronous programming.</li>
</ul>

# List of Endpoints
<ul>
    <li><b>/</b> - <i>Health Check</i>: A simple endpoint to check if the server is running, returns "OK".</li>
    <li><b>/user/login</b> - <i>User Login</i>: Allows a user to log in by checking if their gamer_id exists and verifying their password. If not, the user is added to the database.</li>
    <li><b>/leaderboard</b> - <i>Get Leaderboard</i>: Retrieves the top 5 leaderboard entries, sorted by high score.</li>
    <li><b>/stats/{gamer_id}</b> - <i>Get User Stats</i>: Retrieves the high score and time for a specific user (gamer_id).</li>
    <li><b>/update_stats</b> - <i>Update Stats</i>: Allows updating or inserting a user's high score and time. If the user already exists, the highest score is updated and time is adjusted accordingly.</li>
</ul>
