use crate::component::Component;
use crate::event::Event;
use log::debug;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Write;
use web_view::{Content, WVResult};

#[cfg(feature = "use-local-server")]
use port_check::free_local_port;
#[cfg(feature = "use-local-server")]
use tiny_http::{Header, Response, Server, StatusCode};

pub mod component;
pub mod event;
pub mod icons;
pub mod utils;

#[cfg(debug_assertions)]
const METRO_JS: &str = include_str!("www/js/metro.js");
#[cfg(not(debug_assertions))]
const METRO_JS: &str = include_str!("www/js/metro.min.js");
#[cfg(debug_assertions)]
const METRO_CSS: &str = include_str!("www/css/metro-all.css");
#[cfg(not(debug_assertions))]
const METRO_CSS: &str = include_str!("www/css/metro-all.min.css");

pub struct App {
    title: String,
    content: Box<dyn Component>,
}

impl App {
    pub fn new(title: impl Into<String>, content: impl Component + 'static) -> Self {
        App {
            title: title.into(),
            content: Box::new(content),
        }
    }

    fn build_html(&mut self) -> Result<String, Box<dyn Error>> {
        let html = format!(
            include_str!("www/html/app.html"),
            eventjs = include_str!("www/js/event.js"),
            metrojs = METRO_JS,
            metrocss = METRO_CSS,
            denshicss = include_str!("www/css/denshi.css"),
            content = self.content.render()
        );

        if cfg!(debug_assertions) {
            let _result = remove_file("test.html");
            let mut file = File::create("test.html")?;
            file.write_all(html.as_bytes())?;
        }

        Ok(html)
    }

    fn run_web_view(&mut self, content: Content<String>) -> WVResult {
        let ref title = self.title.clone();

        web_view::builder()
            .content(content)
            .size(800, 600)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                let event: Event = serde_json::from_str(arg).unwrap();
                debug!("Received event {:?}", &event);
                self.content.handle_event(webview, &event);
                Ok(())
            })
            .title(title.as_str())
            .run()
    }

    #[cfg(not(feature = "use-local-server"))]
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let html = self.build_html()?;

        self.run_web_view(Content::Html(html))?;

        Ok(())
    }

    #[cfg(feature = "use-local-server")]
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let html = self.build_html()?;

        let new_port = free_local_port().unwrap();

        dbg!("Using port {}", new_port);

        let html_clone = html.clone();

        let handle = std::thread::spawn(move || {
            let server = Server::http(format!("localhost:{}", new_port)).unwrap();
            for req in server.incoming_requests() {
                let mut resp = Response::new(
                    StatusCode::from(200),
                    Vec::new(),
                    html_clone.as_bytes(),
                    None,
                    None,
                );
                let header =
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8   "[..])
                        .unwrap();
                resp.add_header(header);
                req.respond(resp);
            }
        });

        self.run_web_view(Content::Url(format!(
            "http://localhost:{port}",
            port = new_port
        )))?;

        Ok(())
    }
}
