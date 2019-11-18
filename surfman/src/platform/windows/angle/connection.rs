// surfman/surfman/src/platform/windows/angle/connection.rs
//
//! A connection to the window server.
//! 
//! It might seem like this should wrap an `EGLDisplay`, but it doesn't. Unfortunately, in the
//! ANGLE implementation `EGLDisplay` is not thread-safe, while `surfman` connections must be
//! thread-safe. So we need to use the DXGI/Direct3D concept of a connection instead. These are
//! implicit in the Win32 API, and as such this type is a no-op.

use crate::Error;
use super::device::{Adapter, Device, NativeDevice, VendorPreference};
use super::surface::NativeWidget;

use winapi::shared::minwindef::UINT;
use winapi::shared::windef::HWND;
use winapi::um::d3dcommon::{D3D_DRIVER_TYPE_UNKNOWN, D3D_DRIVER_TYPE_WARP};

#[cfg(feature = "sm-winit")]
use winit::Window;
#[cfg(feature = "sm-winit")]
use winit::os::windows::WindowExt;

const INTEL_PCI_ID: UINT = 0x8086;

/// A no-op connection.
///
/// It might seem like this should wrap an `EGLDisplay`, but it doesn't. Unfortunately, in the
/// ANGLE implementation `EGLDisplay` is not thread-safe, while `surfman` connections must be
/// thread-safe. So we need to use the DXGI/Direct3D concept of a connection instead. These are
/// implicit in the Win32 API, and as such this type is a no-op.
#[derive(Clone)]
pub struct Connection;

/// An empty placeholder for native connections.
///
/// It might seem like this should wrap an `EGLDisplay`, but it doesn't. Unfortunately, in the
/// ANGLE implementation `EGLDisplay` is not thread-safe, while `surfman` connections must be
/// thread-safe. So we need to use the DXGI/Direct3D concept of a connection instead. These are
/// implicit in the Win32 API, and as such this type is a no-op.
#[derive(Clone)]
pub struct NativeConnection;

impl Connection {
    /// Connects to the default display.
    #[inline]
    pub fn new() -> Result<Connection, Error> {
        Ok(Connection)
    }

    /// An alias for `Connection::new()`, present for consistency with other backends.
    #[inline]
    pub unsafe fn from_native_connection(_: NativeConnection) -> Result<Connection, Error> {
        Connection::new()
    }

    /// Returns the underlying native connection.
    #[inline]
    pub fn native_connection(&self) -> NativeConnection {
        NativeConnection
    }

    /// Returns the "best" adapter on this system, preferring high-performance hardware adapters.
    /// 
    /// This is an alias for `Connection::create_hardware_adapter()`.
    #[inline]
    pub fn create_adapter(&self) -> Result<Adapter, Error> {
        self.create_hardware_adapter()
    }

    /// Returns the "best" adapter on this system, preferring high-performance hardware adapters.
    #[inline]
    pub fn create_hardware_adapter(&self) -> Result<Adapter, Error> {
        Adapter::new(D3D_DRIVER_TYPE_UNKNOWN, VendorPreference::Avoid(INTEL_PCI_ID))
    }

    /// Returns the "best" adapter on this system, preferring low-power hardware adapters.
    #[inline]
    pub fn create_low_power_adapter(&self) -> Result<Adapter, Error> {
        Adapter::new(D3D_DRIVER_TYPE_UNKNOWN, VendorPreference::Prefer(INTEL_PCI_ID))
    }

    /// Returns the "best" adapter on this system, preferring software adapters.
    #[inline]
    pub fn create_software_adapter(&self) -> Result<Adapter, Error> {
        Adapter::new(D3D_DRIVER_TYPE_WARP, VendorPreference::None)
    }

    /// Opens the hardware device corresponding to the given adapter.
    /// 
    /// Device handles are local to a single thread.
    #[inline]
    pub fn create_device(&self, adapter: &Adapter) -> Result<Device, Error> {
        Device::new(adapter)
    }

    /// Wraps an ANGLE `EGLDisplay`, along with the associated Direct3D device, in a `Device` and
    /// returns it.
    ///
    /// The underlying `EGLDisplay` is not retained, as there is no way to do this in the EGL API.
    /// Therefore, it is the caller's responsibility to keep it alive as long as this `Device`
    /// remains alive. This function does, however, call `AddRef` on the Direct3D device.
    #[inline]
    pub fn create_device_from_native_device(&self, native_device: NativeDevice)
                                            -> Result<Device, Error> {
        Device::from_native_device(native_device)
    }

    /// Opens the display connection corresponding to the given `winit` window.
    #[cfg(feature = "sm-winit")]
    pub fn from_winit_window(_: &Window) -> Result<Connection, Error> {
        Connection::new()
    }

    /// Creates a native widget type from the given `winit` window.
    /// 
    /// This type can be later used to create surfaces that render to the window.
    #[cfg(feature = "sm-winit")]
    #[inline]
    pub fn create_native_widget_from_winit_window(&self, window: &Window)
                                                  -> Result<NativeWidget, Error> {
        let hwnd = window.get_hwnd() as HWND;
        if hwnd.is_null() {
            Err(Error::IncompatibleNativeWidget)
        } else {
            Ok(NativeWidget { window_handle: hwnd })
        }
    }
}
