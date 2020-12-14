# Zero2Prod

## Digital Ocean

Lots of fun here, but I do typically forget one extremely simple thing: you have to manually (for now) migrate the database!

Run: 

``` sh
DATABASE_URL=<insert your DO db connection string> sqlx migrate run
```
