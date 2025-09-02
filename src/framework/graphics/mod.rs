pub mod graphics_adapter;
mod win_graphics_adapter;

#[cfg(target_os = "windows")]
use crate::framework::graphics::win_graphics_adapter::PlatformGraphicsAdapter;
use crate::framework::Rectangle;

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum SurfaceFormat {
    #[default]
    Color,
    Unknown,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ScanlineOrder {
    #[default]
    Unspecified,
    Progressive,
    UpperField,
    LowerField
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum DisplayModeScaling {
    #[default]
    Unspecified,
    Centered,
    Stretched,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_numerator: u32,
    pub refresh_rate_denominator: u32,
    pub format: SurfaceFormat,
    pub scanline_order: ScanlineOrder,
    pub scaling: DisplayModeScaling
}

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct DisplayModeCollection {
    pub display_modes: Vec<DisplayMode>,
}
#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct GraphicsAdapterOutput {
    pub device_name: String,
    pub desktop_coordinates: Rectangle,
    pub attached_to_desktop: bool,
    pub display_mode_collection: DisplayModeCollection,
    pub current_display_mode: Option<DisplayMode>,
}
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct GraphicsAdapter {
    pub index: u32,
    pub description: String,
    pub device_id: u32,
    pub is_default: bool,
    pub revision: u32,
    pub sub_system_id: u32,
    pub vendor_id: u32,
    pub outputs: Vec<GraphicsAdapterOutput>,
    pub current_output: Option<GraphicsAdapterOutput>,
    
    platform: PlatformGraphicsAdapter
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct GraphicsDevice {
    //pub adapter: Option<GraphicsAdapter>,
    //pub blend_state: BlendState,
    //pub depth_stencil_state: DepthStencilState,
    //pub rasterizer_state: RasterizerState,
    //pub sampler_state_collection: SamplerStateCollection,
    //pub presentation_parameters: PresentationParameters,
    //pub viewport: Viewport,
    //pub render_target: RenderTarget2D,
    //pub swap_chain: SwapChain,
    //pub graphics_profile: GraphicsProfile,
}