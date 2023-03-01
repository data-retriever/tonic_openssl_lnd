// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts four arguments: host, port, cert file, macaroon file
//  cargo run localhost 10006 /home/ash/.polar/networks/1/volumes/lnd/frank/tls.cert /home/ash/.polar/networks/1/volumes/lnd/frank/data/chain/bitcoin/regtest/admin.macaroon

use tonic_openssl_lnd::lnrpc::{OpenChannelRequest};

#[tokio::main]
async fn main() {
  
    // Connecting to LND requires only host, port, cert file, macaroon file
    let mut client = tonic_openssl_lnd::connect_lightning(
        "localhost".to_string(),
        10006,
        "/home/ash/.polar/networks/1/volumes/lnd/frank/tls.cert".to_owned(),
        "/home/ash/.polar/networks/1/volumes/lnd/frank/data/chain/bitcoin/regtest/admin.macaroon"
            .to_owned(),
    )
    .await
    .expect("failed to connect");


    let info = client
        // All calls require at least empty parameter
        .get_info(tonic_openssl_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // let list_of_peers = ListPeersRequest {
    //     latest_error: true,
    // };
    // let result = client
    //     .list_peers(list_of_peers)
    //     .await
    //     .expect("failed to get the list of peers");


    let node_pubkey=hex::decode("039d1b9632c9433380a431369154ca2725d9e5153ebf2227b71e284cdbfd7a5ffe").expect("decoding failed");
    let open_channel_req=OpenChannelRequest{
        sat_per_vbyte:30,
        node_pubkey,
        local_funding_amount:1000000,
        node_pubkey_string:"".to_owned(),
        push_sat:750000,
        target_conf:0,
        sat_per_byte:0,
        private:false,
        min_htlc_msat:100,
        remote_csv_delay:600,
        min_confs:0,
        spend_unconfirmed:true,
        close_address:"bcrt1prnpxwf9tpjm4jll4ts72s2xscq66qxep6w9hf6sqnvwe9t4gvqasklfhyj".to_owned(),
        funding_shim:None,
        remote_max_value_in_flight_msat:880000000,
        remote_max_htlcs:3,
        max_local_csv:900,
        commitment_type:0,
        zero_conf:false,
        scid_alias:false,
        base_fee: 700,
        fee_rate:0,
        use_base_fee:false,
        use_fee_rate:false,
        remote_chan_reserve_sat:20000
    };

    let open_channel_response=client.open_channel(open_channel_req).await.unwrap();;
    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", open_channel_response);
}
