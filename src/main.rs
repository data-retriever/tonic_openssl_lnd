// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts four arguments: host, port, cert file, macaroon file
//  cargo run localhost 10006 /home/ash/.polar/networks/1/volumes/lnd/frank/tls.cert /home/ash/.polar/networks/1/volumes/lnd/frank/data/chain/bitcoin/regtest/admin.macaroon

use tonic_openssl_lnd::lnrpc::{ListPeersRequest, OpenChannelRequest};

#[tokio::main]
async fn main() {
  
    // Connecting to LND requires only host, port, cert file, macaroon file
    let mut client = tonic_openssl_lnd::connect_lightning(
        "localhost".to_string(),
        10004,
        "/home/ash/.polar/networks/1/volumes/lnd/dave/tls.cert".to_owned(),
        "/home/ash/.polar/networks/1/volumes/lnd/dave/data/chain/bitcoin/regtest/admin.macaroon"
            .to_owned(),
    )
    .await
    .expect("failed to connect");

    let info = client
        // All calls require at least empty parameter
        .get_info(tonic_openssl_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    let list_of_peers = ListPeersRequest {
        latest_error: true,
    };
    let result = client
        .list_peers(list_of_peers)
        .await
        .expect("failed to get the list of peers");

    let open_channel_req=OpenChannelRequest{
        sat_per_vbyte:30,
        node_pubkey: b"02b39d3066644b9b552c98533dc3368c974dcc1408ce809057e260b132c530ab6b".to_vec(),
        local_funding_amount:500000,
        node_pubkey_string:"".to_owned(),
        push_sat:500000,
        target_conf:5,
        sat_per_byte:0,
        private:false,
        min_htlc_msat:100,
        remote_csv_delay:50,
        min_confs:5,
        spend_unconfirmed:true,
        close_address:"bcrt1prnpxwf9tpjm4jll4ts72s2xscq66qxep6w9hf6sqnvwe9t4gvqasklfhyj".to_owned(),
        funding_shim:None,
        remote_max_value_in_flight_msat:500000,
        remote_max_htlcs:3,
        max_local_csv:10,
        commitment_type:0,
        zero_conf:true,
        scid_alias:false,
        


    };
    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", result);
}
