use euclid::Transform2D;
use pathfinder_gl::GLDevice;
use style::values::computed::Color;
use usvg::{TextDecorationStyle, Fill, Stroke};

// FIXME
struct PathBuilder;

pub enum AntialiasMode {
    None = 0,
    Gray = 1,
    Subpixel = 2,
    Default = 3,
}

pub enum CompositionOp {
    Over,
}

enum PathState {
    /// Path builder in user-space. If a transform has been applied
    /// but no further path operations have occurred, it is stored
    /// in the optional field.
    UserSpacePathBuilder(PathBuilder, Option<Transform2D<f32>>),
    /// Path builder in device-space.
    DeviceSpacePathBuilder(PathBuilder),
    /// Path in user-space. If a transform has been applied but
    /// but no further path operations have occurred, it is stored
    /// in the optional field.
    UserSpacePath(Path, Option<Transform2D<f32>>),
}

pub struct CanvasData<'a> {
    drawtarget: GLDevice,
    path_state: Option<PathState>,
    state: CanvasPaintState<'a>,
    saved_states: Vec<CanvasPaintState<'a>>,
    webrender_api: webrender_api::RenderApi,
    image_key: Option<webrender_api::ImageKey>,
    /// An old webrender image key that can be deleted when the next epoch ends.
    old_image_key: Option<webrender_api::ImageKey>,
    /// An old webrender image key that can be deleted when the current epoch ends.
    very_old_image_key: Option<webrender_api::ImageKey>,
    pub canvas_id: CanvasId,
}

struct CanvasPaintState<'a> {
    draw_options: DrawOptions,
    fill_syle: Fill,
    stroke_style: Stroke,
    /// The current 2D transform matrix.
    transform: Transform2D<f32>,
    shadow_offset_x: f64,
    shadow_offset_y: f64,
    shadow_blur: f64,
    shadow_color: Color,
}

impl<'a> CanvasPaintState<'a> {
    fn new(antialias: AntialiasMode) -> CanvasPaintState<'a> {
        CanvasPaintState {
            draw_options: DrawOptions::new(1.0, CompositionOp::Over, antialias),
            fill_style: Fill::default(),
            stroke_style: Stroke::default(),
            transform: Transform2D::identity(),
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
            shadow_color: Color::transparent(),
        }
    }
}
