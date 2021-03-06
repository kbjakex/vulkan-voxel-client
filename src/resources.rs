
// At-a-glance view of all resources in the game.
// Should preferably be imported from here for consistency and convenience,
// although in practice there is no difference.

use crate::renderer::renderer::Renderer;

// The main resources struct contains resources shared between
// all states (main menu, settings, game...)
pub struct Resources {
    pub time: core::Time,
    pub window_handle: winit::window::Window,
    pub window_size: core::WindowSize,
    
    pub metrics: metrics::Resources,
    pub renderer: Renderer,
    pub input: input::Resources
}

pub mod core {
    pub struct Time {
        pub at_launch: std::time::Instant, // never updated, measured just before game loop
        pub now: std::time::Instant,       // updated at the very start of each frame
        pub ms_u32: u32,
        pub secs_f32: f32,
    }

    pub struct WindowSize {
        pub extent: erupt::vk::Extent2D,
        pub xy: glam::Vec2, // convenience
        pub monitor_size_px: winit::dpi::LogicalSize<u32>
    }
}

pub mod metrics {
    pub struct FrameTime {
        pub avg_fps: f32,
        pub avg_frametime_ms: f32,
        pub frametime_history: [f32; 16],
        pub last_updated: std::time::Instant,
    }

    pub struct Resources {
        pub frame_count: u32,
        pub frame_time: FrameTime,
    }
}

pub mod input {
    pub struct Resources {
        pub mouse: crate::input::Mouse,
        pub keyboard: crate::input::Keyboard,
        pub settings: crate::input::settings::InputSettings,
        pub clipboard: arboard::Clipboard
    }
}


// Resources specific to the 'game' state, aka
// when you're actually playing and not in a menu
pub mod game_state {
    use crate::game::states::game::input_recorder::InputRecorder;

    pub struct Resources {
        pub username: flexstr::SharedStr,
        pub chat: crate::chat::Chat,
        pub camera: crate::camera::Camera,
        pub net: Net,

        pub input_recorder: InputRecorder
    }

    pub struct Net {
        pub connection: crate::networking::Connection,
        pub network_tick_count: u32,
    }
}