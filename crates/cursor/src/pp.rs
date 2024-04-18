use js_sys::{Reflect, Uint8Array};
use net_shared::{js, js_await};
use tracing::{debug, info, trace, warn};
use wasm_bindgen::prelude::*;
use web_sys::{
	MessageEvent, RtcDataChannel, RtcDataChannelEvent, RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit,
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Port {
	id: u8,
	peer: RtcPeerConnection,
	data_channel: RtcDataChannel,
}

#[wasm_bindgen]
impl Port {
	pub fn new(id: u8) -> Result<Port, JsValue> {
		new(id)
	}

	pub async fn offer(&self) -> RtcSessionDescriptionInit {
		offer(self.id, &self.peer).await
	}
}

fn new(id: u8) -> Result<Port, JsValue> {
	let peer = RtcPeerConnection::new()?;
	let data_channel = peer.create_data_channel("cursor");

	let on_datachannel = Closure::<dyn FnMut(_)>::new(move |event: RtcDataChannelEvent| {
		let chan = event.channel();

		if chan.label() != "cursor" {
			warn!("Peer transmits unknown channel {}. In future versions, such peers will be banned", chan.label());
		}

		let on_message = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
			let js_array = Uint8Array::new(&event.data());
			let msg = js_array.to_vec();
			trace!(packet = ?msg, port = id, "Packet receiver from port {id}");
		});

		chan.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
		on_message.forget();
	});

	peer.set_ondatachannel(Some(on_datachannel.as_ref().unchecked_ref()));
	on_datachannel.forget();

	let this = Port { id, peer, data_channel };

	info!(port = id, "New port {id} constructed");

	Ok(this)
}

async fn offer(id: u8, peer: &RtcPeerConnection) -> RtcSessionDescriptionInit {
	let js_offer = js_await!(peer.create_offer()).unwrap();
	let sdp = Reflect::get(&js_offer, &js!("sdp")).unwrap().as_string().unwrap();
	let mut offer = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
	offer.sdp(&sdp);

	debug!(port = id, ?offer, "Peer on port {id} initiated offer");

	js_await!(peer.set_local_description(&offer)).unwrap();
	offer
}
