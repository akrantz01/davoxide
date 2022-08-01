# DAVOxide
> A simple WebDAV server with a basic web UI, authentication, and permissions.

![Docker Pulls](https://img.shields.io/docker/pulls/akrantz/davoxide?style=for-the-badge)
![Docker Image Version (latest semver)](https://img.shields.io/docker/v/akrantz/davoxide?style=for-the-badge)
![Docker Image Size (latest semver)](https://img.shields.io/docker/image-size/akrantz/davoxide?style=for-the-badge)

DAVOxide provides a simple and lightweight interface to a folder on your server through WebDAV and a simple web UI.
Most operations are intended to be performed over WebDAV such as modifying files or directories.
The web UI exists strictly for permission management and read-only filesystem access.
Permissions can be defined for individual files or entire directories.
Simply specify the path and the desired action, and it will instantly be applied.

Primary authentication is delegated to a SSO proxy such as [Authelia](https://www.authelia.com/).
WebDAV clients can authenticate using HTTP basic authentication with a username and access token.

![](davoxide.png)

## Installation

Docker:

```
docker run -d --name davoxide \
    -e BASE_PATH=/files \
    -e DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
    -v $PWD:/files \
    akrantz/davoxide:latest
```

## Usage

A few motivating and useful examples of how your product can be used. Spice this up with code blocks and potentially more screenshots.

To run DAVOxide, you will need a [Postgres](https://www.postgresql.org/) database instance.
To get started quickly, you can launch one using Docker.

All configuration is done through environment variables. There are only 4 options to configure:
- `RUST_LOG` - The logging configuration (see [env_logger](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) for options)
- `ADDRESS` - The listen address
- `BASE_PATH` - The path files should be served from
- `DATABASE_URL` The database URL to connect to

All options are optional except for `DATABASE_URL`.


## Meta

[Alexander Krantz](https://krantz.dev) – alex@krantz.dev

Distributed under the MIT license. See ``LICENSE`` for more information.

[https://github.com/akrantz01](https://github.com/akrantz01)

### Roadmap

Some future improvements that will be added eventually:

- [ ] Allow sharing single files
- [ ] Multiple access tokens per user
- [ ] User groups

## Contributing

1. Fork it (<https://github.com/akrantz01/davoxide/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

### Development Setup

1. Install Rust, NodeJS, and Docker
2. Start the development containers: `docker compose up -d`
3. Start the development servers:
   - API: `cargo run`
   - UI: `cd frontend && yarn dev`
