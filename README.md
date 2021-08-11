<p align="center">
    <img src="assets/logo.png" width=300><br>
    <img src="https://img.shields.io/badge/client-react-1f87a3?style=for-the-badge&logo=react" style="margin-right:5px">
    <img src="https://img.shields.io/badge/server-rust-b07858?style=for-the-badge&logo=rust" style="margin-right:5px">
</p>

# Kascreech
Kascreech is a real-time multiplayer quiz platform to make learning fun, and certainly not a blatent rip-off of Kahoot. Built in Rust and React, Kascreech is a joint project between myself and [flauntingspade4](https://github.com/flauntingspade4) which allows anyone to play a quiz with their friends. A production build of Kascreech can be found at [kascreech.ga](http://kascreech.ga).

## How to Play
To select a quiz to play, visit [Kahoot's discover page](https://create.kahoot.it/discover). Kascreech has no database, so relies on Kahoot to store the actual quizzes! Once you've selected a quiz, copy its ID from the address bar (it should look something like `fb42b463-e549-4b18-a175-9ca8c510760a`), select "Host" on Kascreech, and enter the ID. Then hit "Create", and you're good to go! From there onwards, everything works exactly like Kahoot.

## How to Run

### Prerequisites
- Cargo and Rust
- NPM and Node
- This repository

Make sure you've set the `REACT_APP_WS_ADDR` environment variable in `client/.env` to wherever you'll be hosting the server, whether that be `localhost` or another IP or address. Ensure the front-end dependencies are installed by running `npm i` in the client directory.

### Running the Development Server
1. Start the front-end by running `npm start` in the client directory.
2. Start the server by running `cargo run` in the server directory. You can pass the address to bind it to as an argument, for example `cargo run -- 0.0.0.0:8000` to run on port 8000. Make sure this is reflected in the client environment variables.

### Deploying to Production
1. Build the React app for production by running `npm run build` in the client directory.
2. Build the Rust back-end by running `cargo build --release` in the server directory.
3. Deploy on your platform of choice. Tungstenite does not support HTTPS on the server, so you'll need a platform that allows you to force HTTP. Netlify and Vercel both force HTTPS, so you won't be able to deploy on either of them. I recommend a low-spec VPS where you can just put a web server (I recommend [Humphrey](https://github.com/w-henderson/Humphrey)) and the server in the same box.

## Tech Stack

### Client
- Written using [React](https://reactjs.org/)
  - [Create React App](https://create-react-app.dev/) toolchain
  - [TypeScript](https://www.typescriptlang.org/)
  - [Sass](https://sass-lang.com/) for styling
  - [Material Community Icons](https://materialdesignicons.com/)
- Hosted using [GCP Compute Engine](https://cloud.google.com/compute)
  - 1x `f1-micro` in `us-east1-b`
- Served using [Humphrey](https://github.com/w-henderson/Humphrey)
  - Cache size of 4MiB
  - All other configuration options default

### Server
- Written in [Rust](https://www.rust-lang.org/)
  - [`tokio-tungstenite`](https://github.com/snapview/tokio-tungstenite) for handling WebSocket connections
  - [`ureq`](https://github.com/algesten/ureq) for ~~stealing~~ requesting information from Kahoot
- Hosted using [GCP Compute Engine](https://cloud.google.com/compute)
  - 1x `f1-micro` in `us-east1-b`

## Screenshots

### Desktop
| Homepage | Lobby |
| --- | --- |
| ![Homepage screenshot](assets/screenshots_desktop/home.png) | ![Lobby screenshot](assets/screenshots_desktop/lobby.png) |

| Question | Leaderboard |
| --- | --- |
| ![Question screenshot](assets/screenshots_desktop/question.png) | ![Leaderboard screenshot](assets/screenshots_desktop/leaderboard.png) |

### Mobile
| Homepage | Question | Result | Placement |
| --- | --- | --- | --- |
| ![Homepage screenshot](assets/screenshots_mobile/home.png) | ![Question screenshot](assets/screenshots_mobile/guess.png) | ![Result screenshot](assets/screenshots_mobile/correct.png) | ![Placement screenshot](assets/screenshots_mobile/end.png) |