use futures_lite::future::{self, block_on};
use mediasoup::{
    data_structures::TransportListenIp,
    router::RouterOptions,
    supported_rtp_capabilities::get_supported_rtp_capabilities,
    webrtc_transport::{TransportListenIps, WebRtcTransportOptions},
    worker::WorkerSettings,
    worker_manager::WorkerManager,
};

async fn run() {
    let worker_manager = WorkerManager::new();
    future::block_on(async move {
        // Create a new worker with default settings
        let worker = worker_manager
            .create_worker(WorkerSettings::default())
            .await
            .unwrap();
        let media_codecs = get_supported_rtp_capabilities().codecs;
        let router = worker
            .create_router(RouterOptions::new(media_codecs))
            .await
            .unwrap();
        let _transport = router
            .create_webrtc_transport(WebRtcTransportOptions::new(TransportListenIps::new(
                TransportListenIp {
                    ip: "127.0.0.1".parse().unwrap(),
                    announced_ip: Some("9.9.9.1".parse().unwrap()),
                },
            )))
            .await;
    });
}

fn main() {
    block_on(run())
}
