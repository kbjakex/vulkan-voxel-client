pub mod assets;
pub mod camera;
pub mod chat;
pub mod core_systems;
pub mod entities;
pub mod game;
pub mod input;
pub mod networking;
pub mod renderer;
pub mod resources;
pub mod text_box;

use game::Game;
use winit::event_loop::EventLoop;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    let event_loop = EventLoop::new();
    let mut game = Game::init(&event_loop).unwrap();
    event_loop.run(move |event, _, flow| game.on_event(event, flow));
    // event_loop.run() does not return
}

/* fn runner() {
    let event_loop = EventLoop::new();



    //core_systems::init(&mut app, &event_loop);

    let mut has_focus = true;

    event_loop.run(move |event, _, flow| {
        if let Err(e) = handle_event(&mut app, event, flow, &mut has_focus) {
            println!("exec_event returned error: {}", e);
        }
    });
}

fn handle_event(app: &mut GameBuilder, event: Event<()>, flow: &mut ControlFlow, has_focus: &mut bool) -> Result<()> {
    match event {
        Event::WindowEvent {
            window_id: _,
            event,
        } => {
            handle_window_event(app, event, flow, has_focus)?;
        }
        Event::MainEventsCleared => {
            let keyboard = &mut *app.world.get_resource_mut::<Keyboard>().unwrap();
            if keyboard.pressed(Key::Escape) {
                *flow = ControlFlow::Exit;
            }

            //println!("FRAME START");
            let now = Instant::now();
            app.update();
            let end = Instant::now();
            //println!("FRAME TOOK {}ms, vs {}ms\n", (end-now).as_secs_f32() * 1000.0, (end - app.world.get_resource::<Time>().unwrap().now).as_secs_f32() * 1000.0);
        }
        Event::LoopDestroyed => {
            app.run_cleanup();
        }
        Event::DeviceEvent { device_id: _, event } if *has_focus => {
            handle_device_event(app, event)?;
        },
        _ => {}
    }
    Ok(())
}

fn handle_device_event(app: &mut GameBuilder, event: DeviceEvent) -> Result<()> {
    let keyboard = &mut *app.world.get_resource_mut::<Keyboard>().unwrap();
    if KeyboardUpdater::handle_key_event(&event, keyboard) {
        return Ok(());
    }

    let mouse = &mut *app.world.get_resource_mut::<Mouse>().unwrap();
    if MouseUpdater::handle_mouse_events(&event, mouse) {
        return Ok(());
    }

    Ok(())
}

fn handle_window_event(app: &mut Game, event: WindowEvent, flow: &mut ControlFlow, has_focus: &mut bool) -> Result<()> {
    let keyboard = &mut *app.world.get_resource_mut::<Keyboard>().unwrap();
    if KeyboardUpdater::handle_window_event(&event, keyboard) {
        return Ok(());
    }

    match event {
        WindowEvent::ScaleFactorChanged { .. } => {} ,
        WindowEvent::Resized(new_size) => {
            let vk = &*app.world.get_resource::<VkContext>().unwrap();
            let extent = vk.swapchain.surface.extent;
            if new_size.width == extent.width && new_size.height == extent.height {
                return Ok(());
            }

            app.window_resized();
        }

        WindowEvent::CloseRequested => {
            *flow = ControlFlow::Exit;
        }

        WindowEvent::Focused(focus_gained) => {
            *has_focus = focus_gained;
        }
        _ => {}
    }
    Ok(())
} */
