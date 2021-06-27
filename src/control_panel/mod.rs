use gtk;
use gtk::prelude::*;
use relm_derive;

use std::sync::{Arc, Mutex};

#[derive(relm_derive::Msg)]
pub enum WinMsg {
    Quit,
}
pub struct Model {
    color: Arc<Mutex<ndarray::Array1<f32>>>,
    expression: Arc<Mutex<ndarray::Array1<f32>>>,
    shape: Arc<Mutex<ndarray::Array1<f32>>>,
}

#[relm_derive::widget]
impl relm::Widget for Column {
    fn model(
        _: &relm::Relm<Self>,
        coefficients: (
            Arc<Mutex<ndarray::Array1<f32>>>,
            Arc<Mutex<ndarray::Array1<f32>>>,
            Arc<Mutex<ndarray::Array1<f32>>>,
        ),
    ) -> Model {
        return Model {
            color: coefficients.0,
            expression: coefficients.1,
            shape: coefficients.2,
        };
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Quit => {
                println!("Msg::Quit");
                gtk::main_quit()
            }
        }
    }

    view! {
        gtk::Window {
            property_default_height: 650,
            property_default_width: 1000,
            title: "Tasks Manager",

            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="color"]
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        label: "Color",
                    },
                    gtk::ScrolledWindow {
                        hexpand: false,
                        vexpand: true,
                        #[name="color_sliders"]
                        gtk::Box {
                            orientation: gtk::Orientation::Horizontal,
                            spacing: 5,

                        },
                    },
                },
                #[name="shape"]
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        label: "Shape",
                    },
                    gtk::ScrolledWindow {
                        hexpand: false,
                        vexpand: true,
                        #[name="shape_sliders"]
                        gtk::Box {
                            orientation: gtk::Orientation::Horizontal,
                            spacing: 5,
                        },
                    },
                },
                #[name="expression"]
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        label: "Expression",
                    },
                    gtk::ScrolledWindow {
                        hexpand: false,
                        vexpand: true,
                        #[name="expression_sliders"]
                        gtk::Box {
                            orientation: gtk::Orientation::Horizontal,
                            spacing: 5,
                        },
                    },
                },

            },
            delete_event(_, _) => (WinMsg::Quit, gtk::Inhibit(false)),
        }
    }

    fn init_view(&mut self) {
        println!("Init view");

        Self::create_sliders(
            -3.0,
            3.0,
            &mut self.widgets.color_sliders,
            &mut self.model.color,
        );
        Self::create_sliders(
            -3.0,
            3.0,
            &mut self.widgets.shape_sliders,
            &mut self.model.shape,
        );
        Self::create_sliders(
            -3.0,
            3.0,
            &mut self.widgets.expression_sliders,
            &mut self.model.expression,
        );
    }

    fn create_sliders(
        min: f64,
        max: f64,
        sliders_container: &mut gtk::Box,
        coefficients: &mut Arc<Mutex<ndarray::Array1<f32>>>,
    ) {
        let num = coefficients.lock().unwrap();

        for index in 0..num.shape()[0] {
            let slider = gtk::ScaleBuilder::new()
                .adjustment(&gtk::Adjustment::new(0.0, min, max, 1.0, 0.0, 0.0))
                .orientation(gtk::Orientation::Vertical)
                .inverted(true)
                .visible(true)
                .build();

            let coefficients = Arc::clone(&coefficients);
            slider.connect_change_value(move |_, _, x| {
                let x = if x < min { min } else { x };
                let x = if x > max { max } else { x };
                let mut num = coefficients.lock().unwrap();
                num[index as usize] = x as f32;
                gtk::Inhibit(false)
            });
            sliders_container.add::<gtk::Scale>(&slider);
        }
    }
}
