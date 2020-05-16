use crate::component::Component;
use crate::event::Event;
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::Write;
use web_view::Content;

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

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        let html = format!(
            include_str!("www/html/app.html"),
            eventjs = include_str!("www/js/event.js"),
            metrojs = METRO_JS,
            metrocss = METRO_CSS,
            denshicss = include_str!("www/css/denshi.css"),
            content = self.content.render()
        );

        if cfg!(debug_assertions) {
            remove_file("test.html")?;
            let mut file = File::create("test.html")?;
            file.write_all(html.as_bytes())?;
        }

        let ref title = self.title.clone();

        web_view::builder()
            .content(Content::Html(html))
            .size(800, 600)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                let event: Event = serde_json::from_str(arg).unwrap();
                dbg!(&event);
                self.content.handle_event(webview, &event);
                Ok(())
            })
            .title(title.as_str())
            .run()?;
        Ok(())
    }
}
