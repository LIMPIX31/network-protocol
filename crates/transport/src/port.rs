use net_shared::{js, js_await, reflect_get, report};
use wasm_bindgen::JsValue;
use web_sys::{RtcConfiguration, RtcDataChannel, RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit};

use crate::{Error, Result};

const CHANNEL_NAME: &str = "transport";

#[derive(Debug)]
pub struct Port {
	peer: RtcPeerConnection,
	channel: RtcDataChannel,
}

impl Port {
	pub(crate) fn new(config: &RtcConfiguration) -> Result<Self> {
		let peer = report!(RtcPeerConnection::new_with_configuration(config), ?config, "Failed to create peer connection")?;

		let channel = peer.create_data_channel(CHANNEL_NAME);

		let this = Self { peer, channel };

		Ok(this)
	}

	pub(crate) async fn offer(&self) -> Result<RtcSessionDescriptionInit> {
		let js_offer = js_await!(self.peer.create_offer())?;
		let offer = build_offer(&js_offer)?;
		js_await!(self.peer.set_local_description(&offer))?;
		Ok(offer)
	}

	pub(crate) async fn accept(&self, offer: &RtcSessionDescriptionInit) -> Result<()> {
		js_await!(self.peer.set_remote_description(offer))?;
		Ok(())
	}
}

fn build_offer(js_offer: &JsValue) -> Result<RtcSessionDescriptionInit> {
	let sdp = reflect_get!(js_offer, "sdp")?.as_string().ok_or(Error::TypeMismatch)?;
	let mut offer = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
	offer.sdp(&sdp);
	Ok(offer)
}
