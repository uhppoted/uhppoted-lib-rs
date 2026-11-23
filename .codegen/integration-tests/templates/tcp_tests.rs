use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::ControllerRecord;
use uhppoted::DoorRecord;
use uhppoted::ListenerRecord;
use uhppoted::StatusRecord;
use uhppoted::CardRecord;
use uhppoted::Transport;

use uhppoted::Alarms;
use uhppoted::Doors;
use uhppoted::Event;

use crate::stub;

const TIMEOUT: Duration = Duration::from_millis(2500);
const DEST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 60003);

{{ range $test := .Tests }}
{{- range .Annotations}}
#[{{ . }}]{{ end }}
fn {{ .Name }}() {
    let stub = stub::tcp();

    let expected = {{ .Expected  }};
{{ range $arg := .Args }}
    let {{ $arg.Name }}{{ $arg.Type }} = {{ $arg.Value }};{{ end }}

    let c = Controller::new(controller).with_address(DEST).with_transport(Transport::Tcp);
    let result = uhppoted::{{ .Function }}({{ template "params" .Args }}).unwrap();

    assert_eq!(result, expected);
}
{{ end }}
{{- define "params"}}&stub.cfg, c{{ range $param := (slice . 1) }}, {{ .Name }}{{ end }}, TIMEOUT{{end}}

#[test]
fn get_card_not_found() {
    let stub = stub::tcp();
    let controller: u32 = 405419896;
    let card: u32 = 10058401;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_card(&stub.cfg, c, card, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Card not found"));
}

#[test]
fn get_card_at_index_not_found() {
    let stub = stub::udp();
    let controller: u32 = 405419896;
    let index: u32 = 136;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_card_at_index(&stub.cfg, c, index, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Card not found"));
}

#[test]
fn get_card_at_index_deleted() {
    let stub = stub::udp();
    let controller: u32 = 405419896;
    let index: u32 = 137;

    let c = Controller::new(controller)
        .with_address(DEST)
        .with_transport(Transport::Tcp);

    let result = uhppoted::get_card_at_index(&stub.cfg, c, index, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Card deleted"));
}
