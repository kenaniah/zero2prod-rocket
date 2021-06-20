# Zero To Production - With Rocket 🚀

[Zero to Production in Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust. This repository provides an alternate implementation using a different set of tools.

## What's Different?

| Original Tool | Alternate Used | Rationale |
| ------------- | -------------- | --------- |
| [Actix](https://actix.rs/) | [Rocket](https://rocket.rs) | Rocket can now run on stable Rust as of v0.5. Rocket requires less boilerplate and may be easier to use.
| [sqlx](https://github.com/launchbadge/sqlx) | [Diesel](https://diesel.rs/) | Diesel provides a query builder and an extensible ORM. Sqlx does not have these features. |

## Docker stuff

```bash
# Build
docker build . -t kenaniah/zero2prod-rocket:latest

# Debug
docker run -it -e DATABASE_URL=postgres://kenaniah@host.docker.internal/zero2prod -p 8000:8000 --entrypoint=/bin/bash kenaniah/zero2prod-rocket
```
