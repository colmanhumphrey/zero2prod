# Zero2Prod

## Digital Ocean

To deploy the app, run:

```sh
doctl apps create --spec spec.yaml
```
In the main folder. This will get the service going.

Lots of fun here, but I do typically forget one extremely simple thing: you have to manually (for now) migrate the database! You can do this once the db is provisioned on DO, then you'll have the connection details:

Run: 

```sh
DATABASE_URL=<insert your DO db connection string> sqlx migrate run
```

We can run `doctl apps list` to get running apps too.

Then for updating, we run:

``` sh
doctl apps update <APP-ID (from list)> --spec=spec.yaml
```

And a new migration if necessary. But note that the app reads from Github!

## Rust

- Install [cargo-audit](https://github.com/RustSec/cargo-audit), since we check that

## Sqlx

When preparing, run `cargo sqlx prepare -- --bin app`. Still needs [`cargo clean`](https://github.com/launchbadge/sqlx/issues/788) first!
