//! This example showcases integrating a tokio-backed task queue into specs.
//!
//! Example modified from: https://docs.rs/specs/0.16.1/specs/index.html

use anyhow::Result;
use crossbeam::queue::SegQueue;
use fixed_vec_deque::FixedVecDeque;
use specs::prelude::*;
use std::{sync::Arc, thread, time};
use tokio::runtime::Runtime;

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
enum SysARequest {
    None,
    Delay(u32),
}

impl Default for SysARequest {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
enum SysAResult {
    None,
    Delay(u32),
}

impl Default for SysAResult {
    fn default() -> Self {
        Self::None
    }
}

struct Tasks<Req, Res>
where
    Req: 'static + Default + Send + Sync,
    Res: 'static + Default + Send + Sync,
{
    result_queue: Arc<SegQueue<Res>>,
    outgoing: FixedVecDeque<[Req; 16]>,
    incoming: FixedVecDeque<[Res; 16]>,
}

impl<Req, Res> Tasks<Req, Res>
where
    Req: 'static + Default + Send + Sync,
    Res: 'static + Default + Send + Sync,
{
    /// Process incoming results from deferred tasks.
    fn process_incoming(&mut self) {
        if !self.incoming.is_full() && !self.result_queue.is_empty() {
            while !self.incoming.is_full() {
                if let Some(result) = self.result_queue.pop() {
                    *self.incoming.push_back() = result;
                    continue;
                }

                break;
            }
        }
    }
}

impl Tasks<SysARequest, SysAResult> {
    fn process(runtime: &Runtime, world: &mut World) {
        // Check if we have any outgoing tasks.
        let mut slf = world.write_resource::<Self>();

        if !slf.outgoing.is_empty() {
            while let Some(outgoing) = slf.outgoing.pop_front() {
                match *outgoing {
                    SysARequest::None => (),
                    SysARequest::Delay(tick) => {
                        let result_queue = slf.result_queue.clone();

                        runtime.spawn(async move {
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                            result_queue.push(SysAResult::Delay(tick));
                        });
                    }
                }
            }
        }

        slf.process_incoming();
    }
}

impl<Req, Res> Default for Tasks<Req, Res>
where
    Req: 'static + Default + Send + Sync,
    Res: 'static + Default + Send + Sync,
{
    fn default() -> Self {
        Self {
            result_queue: Arc::new(SegQueue::new()),
            outgoing: FixedVecDeque::new(),
            incoming: FixedVecDeque::new(),
        }
    }
}

struct SysA {
    task_running: bool,
    tick: u32,
}

impl<'a> System<'a> for SysA {
    type SystemData = (
        Write<'a, Tasks<SysARequest, SysAResult>>,
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
    );

    fn run(&mut self, (mut tasks, mut pos, vel): Self::SystemData) {
        // drain incoming results.
        if !tasks.incoming.is_empty() {
            while let Some(result) = tasks.incoming.pop_front() {
                println!("task completed: {:?}", result);
            }

            self.task_running = false;
        }

        if !self.task_running {
            *tasks.outgoing.push_back() = SysARequest::Delay(self.tick);
            self.tick += 1;
            self.task_running = true;
        }

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }
}

fn main() -> Result<()> {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    world.create_entity().with(Pos(2.0)).build();

    world.insert(Tasks::<SysARequest, SysAResult>::default());

    let mut dispatcher = DispatcherBuilder::new()
        .with(
            SysA {
                task_running: false,
                tick: 0,
            },
            "sys_a",
            &[],
        )
        .build();

    let runtime = Runtime::new()?;

    loop {
        // This dispatches all the systems in parallel.
        dispatcher.dispatch(&mut world);

        Tasks::<SysARequest, SysAResult>::process(&runtime, &mut world);

        thread::sleep(time::Duration::from_millis(50));
    }
}
