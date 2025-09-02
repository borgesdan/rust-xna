use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIOutput};
use crate::exception;
use crate::framework::graphics::GraphicsAdapter;
use crate::shared::{ExceptionConverter, XnaResult};

#[cfg(target_os = "windows")]
#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct WinGraphicsAdapterOutput {
    output: Option<IDXGIOutput>,
}

#[cfg(target_os = "windows")]
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PlatformGraphicsAdapter{
    factory: IDXGIFactory,
    adapter: IDXGIAdapter,
}

#[cfg(target_os = "windows")]
impl GraphicsAdapter {
    pub fn win_adapters() -> XnaResult<Vec<GraphicsAdapter>> {
        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>()
                .unwrap_or_throw(exception!("Failed to created IDXGIFactory.", None))?;

            let mut count : u32 = 0;

            loop {

            }
        }
    }

    pub fn win_create_adapter(factory: &IDXGIFactory, index: u32) -> XnaResult<Option<GraphicsAdapter>> {
        unsafe {
            let adapter = factory.EnumAdapters(index);

            if adapter.is_err() {
                return Ok(None)
            }

            let adapter = adapter?;
            let description = adapter.GetDesc()
                .unwrap_or_throw(exception!("Adapter.GetDescription failed", None))?;

            let adp = GraphicsAdapter {
                index,
                device_id: description.DeviceId,
                is_default: index == 0,
                revision: description.Revision,
                sub_system_id: description.SubSysId,
                vendor_id: description.VendorId,
                description: String::from_utf16(&description.Description).unwrap(),
                platform: PlatformGraphicsAdapter {
                factory: factory.clone(),
                adapter: adapter.clone()
            },
                current_output: None,
                outputs: Vec::new()
            };
            
            Ok(Some(adp))
        }
    }
}