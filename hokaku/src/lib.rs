mod server;

use hokaku_common::PixelFormat;
use obs_wrapper::{
    // Everything required for modules
    prelude::*,
    // Everything required for creating a source
    source::*,
    // Macro for registering modules
    obs_register_module,
    // Macro for creating strings
    obs_string, properties::{Properties, NumberProp},
};

// The module that will handle creating the source.
struct HokakuModule {
    context: ModuleContext
}

// The source that will be shown inside OBS.
struct HokakuSource {
    width: u32,
    height: u32,
    pixel_format: PixelFormat,
}

// The state of the source that is managed by OBS and used in each trait method.
struct SourceData;

// Implement the Sourceable trait for TestSource, this is required for each source.
// It allows you to specify the source ID and type.
impl Sourceable for HokakuSource {
    fn get_id() -> ObsString {
        obs_string!("hokaku_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(create: &mut CreatableSourceContext<Self>, source: SourceContext) -> Self {
        // TODO: everything
        if let Some(a) = create.settings.get_json() {
            println!("{}", a);
        }
        return Self {
            width: 1280,
            height: 720,
            pixel_format: PixelFormat::RGBA
        }
    }
}

impl GetPropertiesSource for HokakuSource {
    fn get_properties(&mut self) -> Properties {
        let mut properties = Properties::new();
        properties
            .add(
                obs_string!("buffer_width"),
                obs_string!("The width of the buffer"),
                NumberProp::new_int().with_range(240..=3840)
            )
            .add(
                obs_string!("buffer_height"),
                obs_string!("The height of the buffer"),
                NumberProp::new_int().with_range(240..=2160)
            );
        properties
    }
}

// Allow OBS to show a name for the source
impl GetNameSource for HokakuSource {
    fn get_name() -> ObsString {
        obs_string!("Hokaku Source")
    }
}

// Implement the Module trait for TestModule. This will handle the creation of the source and
// has some methods for telling OBS a bit about itself.
impl Module for HokakuModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    // Load the module - create all sources, returning true if all went well.
    fn load(&mut self, load_context: &mut LoadContext) -> bool {

        server::start_server(self, load_context);

        // Create the source
        let source = load_context
            .create_source_builder::<HokakuSource>()
            // Since GetNameSource is implemented, this method needs to be called to
            // enable it.
            .enable_get_name()
            .enable_get_properties()
            .with_icon(Icon::Custom)
            .build();

        // Tell OBS about the source so that it will show it.
        load_context.register_source(source);

        // Nothing could have gone wrong, so return true.
        true
    }

    fn description() -> ObsString {
        obs_string!("a very good description")
    }

    fn name() -> ObsString {
        obs_string!("hokaku")
    }

    fn author() -> ObsString {
        obs_string!("Lena")
    }
}

obs_register_module!(HokakuModule);