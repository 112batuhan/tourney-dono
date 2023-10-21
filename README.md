# tourney-dono

A simple discord bot / web server / Nuxt frontend combo to display H1A donations on OBS browser source.

Discord bot is used to manipulate donation data. Webserver to serve the static page files and websocket. Frontend is made with Vue and Nuxt.

This implementation is using PostgreSQL database for storage. A bit overkill but it's easy to develop and deploy.

To run the website. Clone the repo, install Rust, NPM, NodeJS and Docker. Then install Just (Rust package, `cargo install just`) and type `just run` in console. Alternatively, you can build the whole thing in docker using `docker-compose.yml` 

All of the environment variables are read from the justfile in the project root so to run the frontend individually, you need to set the `WS_URL` environment variable. 
