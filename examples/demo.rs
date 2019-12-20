use denshi::App;
use denshi::component::button::Button;
use denshi::component::text::TextField;
use denshi::component::layout::Form;

fn main() {

    let mut form = Form::new();

    let button = Button::new("Test Button".to_owned());
    let text = TextField::new("Enter Name: ".to_owned());

    form.add_line(text);
    form.add_line(button);

    let app = App::new("Demo".to_owned(), form);
    app.run();
}