use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use testconfig::{truncate, TESTDATA, TESTINGDIR};

use mproxy_client::client_socket_stream;
use mproxy_server::listener;

fn test_client(pathstr: &str, listen_addr: String, target_addr: String, tee: bool) {
    let _l = listener(listen_addr, PathBuf::from_str(pathstr).unwrap(), false);
    let _c = client_socket_stream(&PathBuf::from(TESTDATA), vec![target_addr], tee, None);
    let bytesize = truncate(PathBuf::from_str(pathstr).unwrap());
    println!("log size: {}", bytesize);
    assert!(bytesize > 0);
}

#[test]
fn test_client_socket_stream_unicast_ipv4() {
    let pathstr = &[TESTINGDIR, "streamoutput_client_ipv4_unicast.log"].join(&"");
    let listen_addr = "0.0.0.0:9910".to_string();
    let target_addr = "127.0.0.1:9910".to_string();
    test_client(pathstr, listen_addr, target_addr, false)
}

#[test]
fn test_client_socket_stream_multicast_ipv4() {
    let pathstr = &[TESTINGDIR, "streamoutput_client_ipv4_multicast.log"].join(&"");
    let target_addr = "224.0.0.110:9911".to_string();
    let listen_addr = target_addr.clone();
    test_client(pathstr, listen_addr, target_addr, false)
}

#[test]
fn test_client_socket_stream_unicast_ipv6() {
    let pathstr = &[TESTINGDIR, "streamoutput_client_ipv6_unicast.log"].join(&"");
    let listen_addr = "[::1]:9912".to_string();
    let target_addr = "[::1]:9912".to_string();
    test_client(pathstr, listen_addr, target_addr, false)
}

#[test]
fn test_client_socket_stream_multicast_ipv6() {
    let pathstr = &[TESTINGDIR, "streamoutput_client_ipv6_multicast.log"].join(&"");
    let listen_addr = "[ff02::0]:9913".to_string();
    let target_addr = "[ff02::1]:9913".to_string();
    test_client(pathstr, listen_addr, target_addr, false)
}

#[test]
fn test_client_socket_tee() {
    let pathstr = &[TESTINGDIR, "streamoutput_client_tee.log"].join(&"");
    let target_addr = "127.0.0.1:9914".to_string();
    let listen_addr = "0.0.0.0:9914".to_string();
    test_client(pathstr, listen_addr, target_addr, true)
}

#[test]
fn test_client_multiple_servers() {
    let pathstr_1 = &[TESTINGDIR, "streamoutput_client_ipv6_multiplex_1.log"].join(&"");
    let pathstr_2 = &[TESTINGDIR, "streamoutput_client_ipv6_multiplex_2.log"].join(&"");
    let listen_addr_1 = "[::]:9915".to_string();
    let listen_addr_2 = "[::]:9916".to_string();
    let target_addr_1 = "[::1]:9915".to_string();
    let target_addr_2 = "[::1]:9916".to_string();
    //test_client(pathstr, listen_addr, target_addr, false)

    let _l1 = listener(listen_addr_1, PathBuf::from_str(pathstr_1).unwrap(), false);
    let _l2 = listener(listen_addr_2, PathBuf::from_str(pathstr_2).unwrap(), false);
    let _c = client_socket_stream(
        &PathBuf::from(TESTDATA),
        vec![target_addr_1, target_addr_2],
        false,
        None,
    );
    sleep(Duration::from_millis(50));
    let bytesize_1 = truncate(PathBuf::from_str(pathstr_1).unwrap());
    let bytesize_2 = truncate(PathBuf::from_str(pathstr_2).unwrap());
    println!("log sizes: {}, {}", bytesize_1, bytesize_2);
    assert!(bytesize_1 > 0);
    assert!(bytesize_2 > 0);
    assert!(bytesize_1 == bytesize_2);
}
