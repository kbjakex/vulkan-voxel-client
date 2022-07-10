

/* fn update_inputs_first(mut mouse: ResMut<Mouse>, mut keyboard: ResMut<Keyboard>) {
    KeyboardUpdater::tick_keyboard(&mut keyboard);
    MouseUpdater::first_tick(&mut mouse);
}

fn update_inputs_last(mut mouse: ResMut<Mouse>) {
    MouseUpdater::last_tick(&mut mouse);
} */

/* fn update_timings(mut timings: ResMut<FrameTime>, frame_count: Res<FrameCount>) {
    let now = Instant::now();
    let frametime = (now - timings.last_updated).as_secs_f32() * 1000.0;
    timings.frametime_history[frame_count.0 as usize & 15] = frametime;

    let avg = timings.frametime_history.iter().sum::<f32>() / 16.0;
    timings.avg_fps = 1000.0 / avg;
    timings.avg_frametime_ms = avg;
    timings.last_updated = now;
} */

/* fn update_camera(
    mut camera: ResMut<Camera>,
    mouse: Res<Mouse>,
    keyboard: ResMut<Keyboard>,
    mut ui_renderer: ResMut<UiRenderer>,
) {
    if keyboard.is_in_text_input_mode() {
        return;
    }

    let right = keyboard.get_axis(Key::D, Key::A);
    let up = keyboard.get_axis(Key::Space, Key::LShift);
    let fwd = keyboard.get_axis(Key::W, Key::S);

    if right != 0 || up != 0 || fwd != 0 {
        let right_dir = camera.right();
        let fwd_dir = (camera.facing() * Vec3::new(1.0, 0.0, 1.0)).normalize();
        let up_dir = Vec3::Y;

        let velocity = (right as f32) * right_dir + (fwd as f32) * fwd_dir + (up as f32) * up_dir;
        camera.move_by(velocity.normalize() * 0.15);
    }

    camera.update(&*mouse);

    TextRendererUpdater::on_camera_change(ui_renderer.text(), camera.proj_view_matrix());
} */
/* 
fn update_time(mut time: ResMut<Time>) {
    time.now = Instant::now();
    let elapsed = (time.now - time.at_launch).as_nanos() as u64;
    time.ms_u32 = (elapsed / 1_000_000) as u32;
    time.secs_f32 = (elapsed as f64 / 1_000_000_000.0) as f32;
}

pub fn init(game: &mut GameBuilder, event_loop: &EventLoop<()>) {
/*     game.insert_resource(Username("jetp250".to_owned())); // Fiirst.
    game.insert_resource(Clipboard::new().unwrap());

    let networking_stage = networking::init(game);
 */
    // Start by creating window and setting up rendering...
    /* let videomode = event_loop.primary_monitor().unwrap().size();

    let window = WindowBuilder::new()
        //.with_inner_size(PhysicalSize::new(800f32, 800f32))
        .with_inner_size(videomode)
        .with_maximized(true)
        .with_resizable(true)
        .build(&event_loop)
        .unwrap(); */

/*     let render_stage = renderer::init(game, &window);

    let extent = window.inner_size();
    let extent = vk::Extent2D {
        width: extent.width,
        height: extent.height,
    }; */

    game.add_stage(FrameStages::FrameStart, SystemStage::single_threaded())
        .add_stage(
            FrameStages::GameTick,
            SystemStage::single_threaded().with_run_criteria(should_do_gametick),
        )
        .add_stage(FrameStages::Networking, networking_stage)
        .add_stage(FrameStages::EachFrame, SystemStage::single_threaded())
        .add_stage(FrameStages::Render, render_stage)
        .add_stage(FrameStages::FrameEnd, SystemStage::single_threaded())
        .insert_resource(Time {
            at_launch: Instant::now(),
            now: Instant::now(),
            ms_u32: 0,
            secs_f32: 0.0,
        })
        .insert_resource(FrameCount(0))
        .insert_resource(FrameTime{
            avg_fps: 60.0,
            avg_frametime_ms: 16.666,
            frametime_history: [16.666; 16],
            last_updated: Instant::now(),
        })
        .insert_resource(TickCount(0))
        .insert_resource(WindowSize {
            extent,
            xy: Vec2::new(extent.width as f32, extent.height as f32),
        })
        .insert_resource(WindowHandle(window))
        .add_system_to_stage(
            FrameStages::FrameStart,
            update_inputs_first.label(FrameSysLabels::InputUpdate),
        )
        .add_system_to_stage(
            FrameStages::FrameStart,
            update_camera.after(FrameSysLabels::InputUpdate),
        )
        .add_system_to_stage(FrameStages::FrameStart, update_time)
        .add_system_to_stage(FrameStages::FrameStart, update_timings)
        .add_system_to_stage(FrameStages::FrameEnd, update_inputs_last);

    println!("HERE");

    /* input::init(game);
    client::init(game); */
}
 */