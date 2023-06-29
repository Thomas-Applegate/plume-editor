use fltk::{app, prelude::*, window::Window, text::TextEditor,
	text::TextBuffer, enums::Font};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_label("Plume Editor")
		.with_size(500, 600)
		.center_screen();
	wind.size_range(150,100, 0, 0);
    wind.make_resizable(true);
    
    let buf = TextBuffer::default();
    
    let mut txt = TextEditor::default().size_of_parent();
    txt.set_text_font(Font::Courier);
    txt.set_buffer(buf);
    
    wind.end();
    wind.show();
    app.run().unwrap();
}
