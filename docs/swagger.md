# Swagger

Simplify API development for users, teams, and enterprises with the Swagger open source and professional tool set. Find out how Swagger can help you design and document your APIs at scale.

Swagger can be used to define, export and test (maybe?) the API endpoints.

## Installation

The repository includes a `./swagger/docker-compose.yaml` that can be used to spin up a docker stack of swagger-editor.

```sh
docker-compose up -d
```

Once the docker has been pulled down and spun up the editor can be reached on [http://localhost:8080](http://locahost:8080).

## Usage

The swagger definitions file is set by default to `./swagger/swagger.json`. This can be edited with your default editor or swagger-editor.
