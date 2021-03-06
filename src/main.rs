extern crate epoxy;
extern crate gio;
extern crate gl;
extern crate gtk;
extern crate gdk;

use gio::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use std::env::args;

mod renderers;
use self::renderers::basic_renderer::BasicRenderer;
use self::renderers::renderer::Renderer;
//use self::renderers::empty_renderer::EmptyRenderer;

mod gl_loader;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Window");
    window.set_default_size(640, 480);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);

    let gl = std::rc::Rc::new(BasicRenderer::new());

    let gl_area = gtk::GLArea::new();
    gl_area.set_vexpand(true);
    gl_area.set_hexpand(true);
    //gl_area.set_use_es(true);  // undefined
    gl_area.set_required_version(4, 6); // default 3.2

    {
        let gl_clone = gl.clone();
        gl_area.connect_realize(move |gl_area| {
            // setup gl_area
            gl_area.make_current();
            match gl_area.get_error() {
                Some(error) => {
                    println!("gtk::GLArea error: {}", error);
                    return;
                }
                None => {}
            }
            gl_area.set_has_depth_buffer(true);

            match gl_area.get_context() {
                Some(context) =>{

                    let version = context.get_version();
                    println!("context version: {}.{}", version.0, version.1);

                }
                None=>{}
            }

            // initialize opengl
            gl_loader::load();

            // setup scene
            gl_clone.initialize().unwrap();
        });
    }

    {
        let gl_clone = gl.clone();
        gl_area.connect_resize(move |_, w, h| {
            gl_clone.resize(w as u32, h as u32);
        });
    }

    {
        let gl_clone = gl.clone();
        gl_area.connect_render(move |_area, _context| {
            gl_clone.render();
            Inhibit(true)
        });
    }

    {
        let gl_clone = gl.clone();
        window.connect_delete_event(move |win, _| {
            gl_clone.finalize();
            win.destroy();
            Inhibit(false)
        });
    }

    window.add(&gl_area);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
