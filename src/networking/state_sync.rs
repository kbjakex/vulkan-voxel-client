/* use bevy_ecs::{schedule::{SystemStage, ParallelSystemDescriptorCoercion, ShouldRun::{self, Yes, No}}, system::{ResMut, Res}};
use bin_io::reader::BinaryReader;
use shared::{protocol::s2c, TICK_DURATION};

use crate::core_systems::{Time, TickCount};

use super::{IncomingPackets, OutgoingPackets}; */

/* fn sync_entity_state(world: &mut World) {
    world.resource_scope(|world, mut conn: Mut<Connection>|{
        let conn = &mut *conn;

        if let Some(channels) = conn.channels() {
            
        }
    });
}

pub fn init(game: &mut GameBuilder) -> SystemStage {
    let mut graph = SystemGraph::new();

    graph.root(sync_entity_state.exclusive_system());

    SystemStage::single_threaded()
        .with_system_set(graph.into()) */
/*         .with_run_criteria(should_sync)
        .with_system(dummy.label("dummy"))
        .with_system(flush_outgoing.after("dummy")) */
/* } */

/* fn should_sync(time: Res<Time>, ticks: Res<TickCount>) -> ShouldRun {
    let tick_start_time = time.at_launch + ticks.0 * TICK_DURATION;

    if time.now >= tick_start_time {
        Yes
    } else {
        No
    }
}

fn dummy(mut net: ResMut<IncomingPackets>) {
    while let Some(bytes) = net.try_recv() {
        let mut reader = BinaryReader::new(&bytes);

        // Unwrapping is fine because the header has already been parsed once on the network thread
        let header = s2c::read_header(&mut reader).unwrap();

/*         println!(
            "Received packet with id {}, size {}",
            header.id, header.size_bytes
        ); */

        let packet = s2c::ChatMessage::parse(&mut reader).unwrap();
/*         println!("ChatMessage: '{}'", packet.message);
 */
        reader.skip(header.size_bytes);
    }
    //println!("Finished reading packets");
}

fn flush_outgoing(mut outgoing: ResMut<OutgoingPackets>) {
    //println!("Flushing packets");
    outgoing.flush();
}
 */
