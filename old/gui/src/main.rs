use btleplug::{
    api::{Central, Manager as _},
    platform::Manager,
};
use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk::prelude::OrientableExt, prelude::{DynamicIndex, FactoryVecDeque}, FactorySender};
use relm4::prelude::FactoryComponent;
use relm4::{
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, Worker,
    WorkerController, gtk,
};

mod shock;

#[derive(Debug)]
struct BluetoothAdapterInfo {
    index: usize,
    name: String,
}

#[derive(Debug)]
struct BluetoothDeviceInfo {
    index: usize,
    name: String,
}

#[derive(Debug)]
struct AdapterListItem {
    info: BluetoothAdapterInfo,
}

#[derive(Debug)]
enum AdapterListItemInput {}

#[derive(Debug)]
enum AdapterListItemOutput {}

#[relm4::factory]
impl FactoryComponent for AdapterListItem {
    type Init = BluetoothAdapterInfo;
    type Input = AdapterListItemInput;
    type Output = AdapterListItemOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(index_label)]
            gtk::Label {
                #[watch]
                set_label: &self.info.index.to_string(),
                set_width_chars: 3,
            },

            #[name(name_label)]
            gtk::Label {
                #[watch]
                set_label: &self.info.name,
            },

        }
    }
    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { info: value }
    }
}


struct MainApp {
    created_widgets: u8,
    adapters: FactoryVecDeque<AdapterListItem>,
}


#[derive(Debug)]
enum MainAppInput {
    AddAdapter,
    RemoveAdapter,
}

#[relm4::component]
impl SimpleComponent for MainApp {
    type Init = ();
    type Input = MainAppInput;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Coyote Control"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Add adapter",
                    connect_clicked => MainAppInput::AddAdapter,
                },

                gtk::Button {
                    set_label: "Remove adapter",
                    connect_clicked => MainAppInput::RemoveAdapter,
                },

                #[local_ref]
                adapter_list_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    // Initialize the UI.
    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let adapters = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = MainApp {
            created_widgets: 0,
            adapters,
        };

        let adapter_list_box = model.adapters.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            MainAppInput::AddAdapter => {
                self.adapters.guard().push_back(BluetoothAdapterInfo {
                    index: self.created_widgets as usize,
                    name: format!("Adapter {}", self.created_widgets),
                });
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            MainAppInput::RemoveAdapter => {
                self.adapters.guard().pop_back();
            }
        }
    }
}


fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<MainApp>(());
}
