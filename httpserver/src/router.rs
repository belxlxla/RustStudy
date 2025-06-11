use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest::{HttpRequest, Resource}, httpresponse::HttpResponse};
use http::Method;
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    if route.len() > 1 {
                        match route[1] {
                            "api" => {
                                let resp: HttpResponse = WebServiceHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            }
                            _ => {
                                let resp: HttpResponse = StaticPageHandler::handle(&req);
                                let _ = resp.send_response(stream);
                            }
                        }
                    } else {
                        let resp: HttpResponse = StaticPageHandler::handle(&req);
                        let _ = resp.send_response(stream);
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}