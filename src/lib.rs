use worker::*;

mod utils;

async fn retrieve_file_from_bucket(key: &str, env: &Env) -> Result<Response> {
    let bucket: Bucket = env.bucket("FILES")?;

    let mut headers: Headers = Headers::new();

    let opt: GetOptionsBuilder = bucket.get(key);
    return match opt.execute().await? {
        Some(object) => {
            set_headers(object.http_metadata(), &mut headers);
            Ok(Response::from_bytes(object.body().unwrap().bytes().await?)?.with_headers(headers))
        },
        None => {
            utils::log_not_present_error("FILES", key);
            Response::error(
                utils::create_error_response("Not Found", "404 Not Found", "Oops, looks like we weren't able to find the file you were looking for."),
                404
            )
        }
    }
}

#[allow(unused_must_use)]
fn set_headers(metadata: HttpMetadata, headers: &mut Headers) {
    if let Some(content_type) = metadata.content_type {
        headers.set("Content-Type", &content_type);
    }
    if let Some(content_encoding) = metadata.content_encoding {
        headers.set("Content-Encoding", &content_encoding);
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello from the vault"))
        .get_async("/file/:key", |_, ctx| async move {
            if let Some(key) = ctx.param("key") {
                return match retrieve_file_from_bucket(key, &ctx.env).await {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        utils::log_generic_error(key, &e.to_string());
                        // Generic error message
                        Response::error(utils::create_error_response("Bad Request", "500 Internal Server Error", "Sorry, something went wrong and we're unable to handle your request."), 500)
                    }
                };
            } else {
                // No key - bad client request
                return Response::error(utils::create_error_response("Bad Request", "400 Bad Request", "Looks like that's not a valid file!"), 400);
            }
        })
        .run(req, env)
        .await
}
