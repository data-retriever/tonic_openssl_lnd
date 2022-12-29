// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// This program accepts four arguments: host, port, cert file, macaroon file
//  cargo run localhost 10006 /home/ash/.polar/networks/1/volumes/lnd/frank/tls.cert /home/ash/.polar/networks/1/volumes/lnd/frank/data/chain/bitcoin/regtest/admin.macaroon

use tonic_openssl_lnd::lnrpc::ListPeersRequest;

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

    let list_of_peers = ListPeersRequest {
        latest_error: true,
    };
    let result = client
        .list_peers(list_of_peers)
        .await
        .expect("failed to get the list of peers");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", result);
}
