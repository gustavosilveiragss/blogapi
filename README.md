# Blog API: Actix Web API with Diesel and PostgreSQL

This is the repo for the API used in my [personal blog](https://github.com/gustavosilveiragss/blog).

Made mainly to learn more about Actix Web and integrating it with a React frontend.

Technologies used:
- Rust
- Actix Web
- Diesel ORM
- PostgreSQL (currently hosted on Supabase, but that might change)
- Docker
- Google Cloud Run

## Adding new services

Go to `services.rs` and add a new public async function with the `get()` or `post()` macro, following the pattern for other routes. If it requires arguments in the url, add `path: Path<T>` in the functions's arguments.

In `messages.rs` create a new public struct with the `Message` macro and whatever else that may need. Also add a `#[rtype(result = "QueryResult<T>")]` with the `QueryResult`'s Payload Model (checkout `payload_models.rs` to add a new one) or plain Database Model (from `db_models.rs`). If that route takes arguments, add the arguments needed as public inside the struct.

Then, in `actors.rs` implement a new `Handler<[MESSAGE]>` for the `DbActor`. In the `handle()` function, only use `msg` if the query uses arguments. Personalize the Diesel query with whatever it is you wanna do.
Finally, add the service to the app in `main.rs` with `.service([SERVICE])`. And the new route should be functional.

If you're making a `POST` query, just look for how I did it with other routes and it should be simple to figure out the few differences.

## Docker and Deploying

I'm not particularly great with Docker, so the way I did it is probably not optimal. But this is how you'd build and deploy to Google Cloud Run.

```Bash
cargo build --release --target x86_64-unknown-linux-gnu

docker build -t gcr.io/[PROJECT_ID]/blogapi .

gcloud run deploy --image gcr.io/[PROJECT_ID]/blogapi --platform managed
```

There might be some issues with env variables but just set them inside the Google Cloud Console itself.

## Conclusion

This was a fun project to build and quite fulfilling. I'm sure there are lots of refactoring to be made and some of it seems too boiler plate.

I was hosting the previous NextJS API in Vercel, but in the same region as the current host. 

The average response time for the `fetch all` query with the NextJS API was ~600ms.

Current average responde time for that query is ~40ms.

[BLAZINGLY FAST](https://i.redd.it/kczaqedt9ww81.jpg)