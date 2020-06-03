use crate::component::Component;
use crate::event::{Event, EventBroker};
use log::{debug, info};
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Write;
use web_view::{Content, WebView};

#[cfg(feature = "use-local-server")]
use port_check::free_local_port;
#[cfg(feature = "use-local-server")]
use tiny_http::{Header, Response, Server, StatusCode};
use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;
use crate::AppError::NoAppContentError;
use std::fmt;

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

/// The main application
#[derive(Clone)]
pub struct App {
    title: String,
    content: Rc<RefCell<Option<Box<dyn Component>>>>,
    event_broker: Rc<RefCell<EventBroker>>,
}

/// Errors to be returned from app functions
#[derive(Debug)]
pub enum AppError {
    NoAppContentError,
}

impl Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No content defined for this app.")
    }
}

impl App {
    /// Create a new application with a given title.
    pub fn new(title: impl Into<String>) -> Self {
        App {
            title: title.into(),
            content: Rc::new(RefCell::new(None)),
            event_broker: Rc::new(RefCell::new(EventBroker::new())),
        }
    }

    /// Set the main content of the app.
    pub fn set_content(&self, content: impl Component + 'static) {
        self.content.borrow_mut().replace(Box::new(content));
    }

    /// Send an event to all components of the app.
    pub fn send<E: Any>(&self, webview: &mut WebView<()>, event: &E) {
        self.event_broker.borrow().send(webview, event)
    }

    /// Subscribe to an event.
    pub fn subscribe<F: Fn(&mut WebView<()>, &E) + 'static, E: Any>(&self, listener: F) {
        self.event_broker.borrow_mut().subscribe(listener)
    }

    fn build_html(&mut self) -> Result<String, Box<dyn Error>> {
        let mut content = self.content.borrow_mut();

        if content.is_none() {
            return Err(Box::new(NoAppContentError));
        }

        let html = format!(
            include_str!("www/html/app.html"),
            eventjs = include_str!("www/js/event.js"),
            metrojs = METRO_JS,
            metrocss = METRO_CSS,
            denshicss = include_str!("www/css/denshi.css"),
            content = content.as_mut().unwrap().render()
        );

        if cfg!(debug_assertions) {
            let _result = remove_file("test.html");
            let mut file = File::create("test.html")?;
            file.write_all(html.as_bytes())?;
        }

        Ok(html)
    }

    fn run_web_view(&mut self, content_str: Content<String>) -> Result<(), Box<dyn Error>> {
        let ref title = self.title.clone();
        let mut content = self.content.borrow_mut();

        if content.is_none() {
            return Err(Box::new(NoAppContentError));
        }

        web_view::builder()
            .content(content_str)
            .size(800, 600)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                let event: Event = serde_json::from_str(arg).unwrap();
                debug!("Received event {:?}", &event);
                content.as_mut().unwrap().handle_event(webview, &event);
                Ok(())
            })
            .title(title.as_str())
            .run()?;
        Ok(())
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

        info!("Using port {}.", new_port);

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
