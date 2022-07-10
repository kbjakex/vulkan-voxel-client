pub mod input_recorder;

use std::ffi::c_void;

use erupt::vk::{self, BufferUsageFlags};
use flexstr::SharedStr;
use glam::{Mat4, Vec2, Vec3};
use shared::protocol::s2c::login::LoginResponse;
use vkcore::{Buffer, BufferAllocation, UsageFlags, VkContext};
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent};

use crate::{
    camera::Camera,
    chat::{self, Chat},
    game::{State, StateChange},
    input::Key,
    networking::Connection,
    renderer::{
        passes::terrain_pass::Vertex, renderer::Clear, ui_renderer::UiRenderer,
        wrappers::VertexBuffer, text_renderer::TextColor,
    },
    resources::{core::WindowSize, game_state::{self, Net}, Resources},
};

use self::input_recorder::InputRecorder;

use super::connection_lost::ConnectionLostState;

pub struct GameState {
    pub resources: game_state::Resources,

    grid_vbo: VertexBuffer,
}

impl State for GameState {
    fn on_enter(&mut self, res: &mut Resources) -> anyhow::Result<()> {
        let size = res
            .window_handle
            .primary_monitor()
            .unwrap()
            .size()
            .to_logical::<u32>(res.window_handle.scale_factor());
        println!("Window size: {size:?}");
        res.window_handle.set_inner_size(size);
        res.window_handle.set_maximized(true);
        println!("Entering GameState");

        self.grid_vbo = create_debug_grid(&mut res.renderer.vk)?;
        res.renderer
            .vk
            .uploader
            .flush_staged(&res.renderer.vk.device)?;

        dbg![res.input.keyboard.is_in_text_input_mode()];

        Ok(())
    }

    fn on_update(&mut self, res: &mut Resources) -> Option<Box<StateChange>> {
        self.update_resources(res);
        
        self.update_net(res);
        if self.resources.net.connection.closed() {
            return Some(Box::new(StateChange::SwitchTo(Box::new(
                ConnectionLostState::new(),
            ))));
        }

        let h = res.window_size.extent.height as u16;
        res.renderer.ui.draw_rect_xy_wh((8, h-90), (350, h-8), 0x22_22_22_88);
        res.renderer.ui.draw_text_colored(
            &format!("Text mode: {}", res.input.keyboard.is_in_text_input_mode()),
            10,
            h - 30,
            TextColor::from_rgba32(0xFF_FF_FF_FF)
        );
        let pos = self.resources.camera.pos();
        res.renderer.ui.draw_text_colored(
            &format!("Pos: {:#.3}, {:#.3}, {:#.3}", pos.x, pos.y, pos.z),
            10,
            h - 60,
            TextColor::from_rgba32(0xFF_FF_FF_FF)
        );

        // TODO...
        chat::process_text_input(
            &mut res.input.keyboard,
            &mut res.input.mouse,
            &mut self.resources.chat,
            &mut res.renderer.ui,
            &mut self.resources.net.connection,
            &mut res.input.clipboard,
            &res.window_size,
            &res.window_handle,
            res.time.secs_f32,
        );

        if let Err(e) = self.render(res) {
            eprintln!("render() error: {e}");
        }
        None
    }

    fn on_exit(&mut self, _res: &mut Resources) -> anyhow::Result<()> {
        println!("Exiting GameState");
        self.resources.net.connection.send_disconnect();
        Ok(())
    }

    fn on_event(&mut self, event: &Event<()>, res: &mut Resources) -> Option<Box<StateChange>> {
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                self.resources.camera.on_window_resize(res.window_size.xy);
                self.resources
                    .chat
                    .on_window_resize(res.window_size.extent.width as _, res.renderer.ui.text());
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if !self.resources.chat.is_open() {
                    let speed = res.input.settings.mouse_sensitivity * 0.0025;
                    self.resources
                        .camera
                        .rotate(delta.0 as f32 * speed, delta.1 as f32 * speed);
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(Key::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                return Some(Box::new(StateChange::Exit));
            }
            _ => {}
        }
        None
    }
}

// Networking
impl GameState {
    fn update_net(&mut self, res: &mut Resources) {
        let net = &mut self.resources.net;

        net.connection.tick();

        if res.time.ms_u32 > net.network_tick_count * shared::TICK_DURATION.as_millis() as u32 {
            net.network_tick_count += 1;
            self.send_stuff_to_server(res);
        }
    }

    fn send_stuff_to_server(&mut self, res: &mut Resources) {
        let net = &mut self.resources.net;

        let velocity = self.resources.input_recorder.integrator.end_network_tick();
        dbg![velocity];

        if let Some(channels) = net.connection.channels() {
            let mut payload = vec![0u8; 256];            
            let ints : &mut [u32] = bytemuck::cast_slice_mut(&mut payload);

            
        }
    }
}

// Resources
impl GameState {
    fn update_resources(&mut self, res: &mut Resources) {
        self.update_camera(res);
    }

    fn update_camera(&mut self, res: &mut Resources) {
        let keyboard = &mut res.input.keyboard;
        let mouse = &mut res.input.mouse;
        let camera = &mut self.resources.camera;

        let keyboard = if keyboard.is_in_text_input_mode() {
            None
        } else {
            Some(keyboard)
        };

        let pos = self.resources.input_recorder.integrator.update(
            keyboard,
            camera,
            2.0,
            res.time.secs_f32,
        );

        camera.move_by(pos - camera.pos());
        camera.update(mouse);
    }
}

impl GameState {
    fn draw_crosshair(ui: &mut UiRenderer, win_size: &WindowSize) {
        let (w, h) = (win_size.extent.width as u16, win_size.extent.height as u16);
        ui.draw_rect_xy_wh((w / 2 - 12, h / 2 - 1), (24, 2), 0x99_99_99_FF);
        ui.draw_rect_xy_wh((w / 2 - 1, h / 2 - 12), (2, 24), 0x99_99_99_FF);
    }

    fn render(&mut self, res: &mut Resources) -> anyhow::Result<()> {
        Self::draw_crosshair(&mut res.renderer.ui, &res.window_size);

        chat::draw_chat(
            &mut self.resources.chat,
            res.time.secs_f32,
            &mut res.renderer.ui,
            &res.window_size,
        );

        let renderer = &mut res.renderer;
        let ctx = renderer.start_frame()?;

        let vk = &mut renderer.vk;
        let passes = &renderer.state.render_passes;

        UiRenderer::do_uploads(&mut renderer.ui, vk, ctx.frame)?;

        ctx.render_pass(
            &vk.device,
            &passes.terrain,
            0,
            Clear::ColorAndDepth([0.3, 0.5, 0.8], 0.0),
            || unsafe {
                vk.device.cmd_bind_pipeline(
                    ctx.commands,
                    vk::PipelineBindPoint::GRAPHICS,
                    renderer.state.pipelines.terrain.handle,
                );
                let pv = self.resources.camera.proj_view_matrix();
                let pvm_ptr = &pv as *const Mat4 as *const c_void;
                vk.device.cmd_push_constants(
                    ctx.commands,
                    renderer.state.pipelines.terrain.layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    std::mem::size_of::<Mat4>() as u32,
                    pvm_ptr,
                );
                vk.device.cmd_bind_descriptor_sets(
                    ctx.commands,
                    vk::PipelineBindPoint::GRAPHICS,
                    renderer.state.pipelines.terrain.layout,
                    0,
                    &[renderer.state.descriptors.textures.descriptor_set],
                    &[],
                );
                vk.device.cmd_bind_vertex_buffers(
                    ctx.commands,
                    0,
                    &[self.grid_vbo.buffer.handle],
                    &[0],
                );
                vk.device
                    .cmd_draw(ctx.commands, self.grid_vbo.vertex_count, 1, 0, 0);
            },
        );

        ctx.render_pass(&vk.device, &passes.luma, 0, Clear::None, || unsafe {
            vk.device.cmd_bind_pipeline(
                ctx.commands,
                vk::PipelineBindPoint::GRAPHICS,
                renderer.state.pipelines.luma.handle,
            );
            vk.device.cmd_bind_descriptor_sets(
                ctx.commands,
                vk::PipelineBindPoint::GRAPHICS,
                renderer.state.pipelines.luma.layout,
                1,
                &[renderer.state.descriptors.attachments.luma_descriptor_set],
                &[],
            );

            vk.device.cmd_draw(ctx.commands, 3, 1, 0, 0);
        });
        ctx.render_pass(
            &vk.device,
            &passes.fxaa,
            ctx.swapchain_img_idx,
            Clear::Color(0.0, 0.0, 0.0),
            || unsafe {
                vk.device.cmd_bind_pipeline(
                    ctx.commands,
                    vk::PipelineBindPoint::GRAPHICS,
                    renderer.state.pipelines.fxaa.handle,
                );
                vk.device.cmd_bind_descriptor_sets(
                    ctx.commands,
                    vk::PipelineBindPoint::GRAPHICS,
                    renderer.state.pipelines.fxaa.layout,
                    1,
                    &[renderer.state.descriptors.attachments.fxaa_descriptor_set],
                    &[],
                );

                vk.device.cmd_draw(ctx.commands, 3, 1, 0, 0);
            },
        );
        ctx.render_pass(
            &vk.device,
            &passes.ui.game,
            ctx.swapchain_img_idx,
            Clear::None,
            || {
                UiRenderer::render(
                    &mut renderer.ui,
                    &vk.device,
                    &ctx,
                    &renderer.state.pipelines,
                    &renderer.state.descriptors,
                    res.window_size.xy,
                );
            },
        );

        renderer.end_frame(ctx);
        Ok(())
    }
}

// Initialization
impl GameState {
    pub fn init(
        username: SharedStr,
        login: LoginResponse,
        connection: Connection,
        res: &mut Resources,
    ) -> GameState {
        Self {
            resources: game_state::Resources {
                username,
                chat: Chat::new(res.window_size.extent.width as _),
                net: game_state::Net {
                    connection,
                    network_tick_count: 0,
                },
                camera: Camera::new(login.position, res.window_size.xy),
                input_recorder: InputRecorder::new(login.position, res.time.secs_f32),
            },
            grid_vbo: VertexBuffer {
                buffer: Buffer::null(),
                vertex_count: 0,
            },
        }
    }
}

fn create_debug_grid(vk: &mut VkContext) -> anyhow::Result<VertexBuffer> {
    let mut vertices: Vec<Vertex> = Vec::new();

    for x in -50..50 {
        for z in -50..50 {
            let (x, z) = (x as f32, z as f32);
            vertices.push(Vertex {
                pos: Vec3::new(x, 0.0, z),
                col: Vec3::ZERO,
                uv: (Vec2::new(x, z) / 100.0 + 0.5) * 100.0 / 16.0,
            });
            vertices.push(Vertex {
                pos: Vec3::new(x, 0.0, z + 1.0),
                col: Vec3::ZERO,
                uv: (Vec2::new(x, z + 1.0) / 100.0 + 0.5) * 100.0 / 16.0,
            });
            vertices.push(Vertex {
                pos: Vec3::new(x + 1.0, 0.0, z),
                col: Vec3::ZERO,
                uv: (Vec2::new(x + 1.0, z) / 100.0 + 0.5) * 100.0 / 16.0,
            });

            vertices.push(Vertex {
                pos: Vec3::new(x + 1.0, 0.0, z),
                col: Vec3::ZERO,
                uv: (Vec2::new(x + 1.0, z) / 100.0 + 0.5) * 100.0 / 16.0,
            });
            vertices.push(Vertex {
                pos: Vec3::new(x, 0.0, z + 1.0),
                col: Vec3::ZERO,
                uv: (Vec2::new(x, z + 1.0) / 100.0 + 0.5) * 100.0 / 16.0,
            });
            vertices.push(Vertex {
                pos: Vec3::new(x + 1.0, 0.0, z + 1.0),
                col: Vec3::ZERO,
                uv: (Vec2::new(x + 1.0, z + 1.0) / 100.0 + 0.5) * 100.0 / 16.0,
            });
        }
    }

    let mut buffer = vk.allocator.allocate_buffer(
        &vk.device,
        &BufferAllocation {
            size: vertices.len() * std::mem::size_of::<Vertex>(),
            usage: UsageFlags::FAST_DEVICE_ACCESS,
            vk_usage: BufferUsageFlags::VERTEX_BUFFER,
        },
    )?;

    vk.uploader
        .upload_to_buffer(&vk.device, &vertices[..], &mut buffer, 0)?;

    Ok(VertexBuffer {
        buffer,
        vertex_count: vertices.len() as u32,
    })
}
