use rust_embed::RustEmbed;
use std::borrow::Cow;
use warp::http::header::HeaderValue;
use warp::Filter;
use hyper::Body;

// #[derive(RustEmbed)] macro'su, bu struct'a 'get' gibi statik metodlar ekler.
#[derive(RustEmbed)]
#[folder = "web/dist/"]
struct WebAssets;

/// Statik web arayüzü dosyalarını sunan bir Warp filter'ı oluşturur.
pub fn static_files() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::tail())
        .and_then(serve_static_files)
}

async fn serve_static_files(path: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let path = if path.as_str().is_empty() {
        "index.html"
    } else {
        path.as_str()
    };
    
    // DÜZELTME: Doğrudan ve en basit şekilde `get` metodunu çağırıyoruz.
    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            let body = Body::from(content.data);
            let mut res = warp::reply::Response::new(body);
            res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
            Ok(res)
        }
        None => match WebAssets::get("index.html") {
            Some(content) => {
                let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                let body = Body::from(content.data);
                let mut res = warp::reply::Response::new(body);
                res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
                Ok(res)
            }
            None => Err(warp::reject::not_found()),
        },
    }
}