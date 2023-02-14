use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{Blobstore, BlobstoreSender, Chunk, PutObjectRequest};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct WasmBlobstoreActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for WasmBlobstoreActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    /// The value is also saved to a blobstore (filesystem)
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let content = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "name")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "WebExpo".to_string());

        let filename = format!("{}.txt", content);

        // make directory
        let directory = "webexpo2023";
        let blobstore = BlobstoreSender::new();
        blobstore
            .create_container(ctx, &directory.to_string())
            .await?;

        // upload file
        blobstore
            .put_object(
                ctx,
                &PutObjectRequest {
                    chunk: Chunk {
                        object_id: filename,
                        container_id: directory.to_string(),
                        bytes: content.as_bytes().to_vec(),
                        offset: 0,
                        is_last: true,
                    },
                    content_type: Some("text/plain".to_string()),
                    ..Default::default()
                },
            )
            .await?;

        Ok(HttpResponse {
            body: format!("Hello {}", content).as_bytes().to_vec(),
            ..Default::default()
        })
    }
}
