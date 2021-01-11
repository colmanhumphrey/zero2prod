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
