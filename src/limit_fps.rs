use std::time::{Duration, Instant};

use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSet},
};
pub struct LimitFPS;

impl Plugin for LimitFPS {
    fn build(&self, app: &mut App) {
        // app //
        //     .insert_resource(FrameTimer::default());

        app.sub_app_mut(RenderApp) //
            .insert_resource(FrameTimer::default())
            .add_systems(
                Render,
                framerate_limiter
                    .in_set(RenderSet::Cleanup)
                    .after(World::clear_entities),
            );
    }
}

/// Tracks the instant of the end of the previous frame.
#[derive(Debug, Clone, Resource, Reflect)]
pub struct FrameTimer {
    sleep_end: Instant,
}

impl Default for FrameTimer {
    fn default() -> Self {
        FrameTimer {
            sleep_end: Instant::now(),
        }
    }
}

fn framerate_limiter(mut timer: ResMut<FrameTimer>, mut oversleep: Local<Duration>) {
    let frame_time = timer.sleep_end.elapsed();
    let limit_framerate = Duration::from_secs_f32(1. / 60.);

    let sleep_time = limit_framerate.saturating_sub(frame_time + *oversleep);

    spin_sleep::sleep(sleep_time);

    let frame_time_total = timer.sleep_end.elapsed();
    timer.sleep_end = Instant::now();

    *oversleep = frame_time_total.saturating_sub(limit_framerate);
    // info!("oversleep: {:?}", *oversleep);
}
