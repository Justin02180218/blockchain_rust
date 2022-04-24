use libp2p::{
    NetworkBehaviour, 
    gossipsub::{Gossipsub, GossipsubConfig, MessageAuthenticity, GossipsubEvent}, 
    mdns::{Mdns, MdnsEvent}, 
    identity::Keypair, 
    swarm::NetworkBehaviourEventProcess
};
use anyhow::Result;
use tokio::sync::mpsc;
use tracing::{info, error};

use crate::Messages;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct BlockchainBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub msg_sender: mpsc::UnboundedSender<Messages>,
}

impl BlockchainBehaviour {
    pub async fn new(key_pair: Keypair, config: GossipsubConfig, msg_sender: mpsc::UnboundedSender<Messages>) -> Result<Self> {
        Ok(Self {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(key_pair), config).unwrap(),
            mdns: Mdns::new(Default::default()).await?,
            msg_sender,
        })
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => {
                info!("Got message with id: {} from peer: {:?}", id, peer_id);
                let msg: Messages = serde_json::from_slice(&message.data).unwrap();
                if let Err(e) = self.msg_sender.send(msg) {
                    error!("error sending messages via channel, {}", e);
                }
            },
            GossipsubEvent::Subscribed { peer_id, topic} => {
                info!("Subscribe topic: {} with id: {}", topic, peer_id);
            },
            GossipsubEvent::Unsubscribed { peer_id, topic} => {
                info!("Unsubscribe topic: {} with id: {}", topic, peer_id);
            },
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (id, addr) in list {
                    println!("Got peer: {} with addr {}", &id, &addr);
                    self.gossipsub.add_explicit_peer(&id);
                }
            },
            MdnsEvent::Expired(list) => {
                for (id, addr) in list {
                    println!("Removed peer: {} with addr {}", &id, &addr);
                    self.gossipsub.remove_explicit_peer(&id);
                }
            }
        }
    }
}