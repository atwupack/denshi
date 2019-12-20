use web_view::Content;
use crate::component::Component;
use crate::event::Event;

pub mod component;
pub mod event;

pub struct App {
    title: String,
    content: Box<dyn Component>,
}


impl App {
    pub fn new(title: String, content: impl Component + 'static) -> Self {
        App {
            title,
            content: Box::new(content),
        }
    }

    pub fn run(mut self) {

        let html = format!(include_str!("www/html/app.html"),
                           content = self.content.render(),
                           event = include_str!("www/js/event.js"));

        let ref title = self.title.clone();

        web_view::builder()
            .content(Content::Html(html))
            .size(800, 600)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|_webview, arg| {
                let event: Event = serde_json::from_str(arg).unwrap();
                dbg!(&event);
                self.content.handle_event(&event);
                Ok(())
            })
            .title(title.as_str())
            .run()
            .unwrap();
    }
}