use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::ControllerRecord;
use uhppoted::DoorRecord;
use uhppoted::ListenerRecord;
use uhppoted::StatusRecord;
use uhppoted::CardRecord;

use uhppoted::Alarms;
use uhppoted::Doors;
use uhppoted::Event;

use crate::stub;

const TIMEOUT: Duration = Duration::from_millis(2500);
{{ range $test := .Tests }}
{{- range .Annotations}}
#[{{ . }}]{{ end }}
fn {{ .Name }}() {
    let stub = stub::broadcast();

    let expected = {{ .Expected  }};
{{ range $arg := .Args }}
    let {{ $arg.Name }}{{ $arg.Type }} = {{ $arg.Value }};{{ end }}

    let result = uhppoted::{{ .Function }}({{ template "params" .Args }}).unwrap();

    assert_eq!(result, expected);
}
{{ end }}
{{- define "params"}}&stub.cfg{{ range $param := . }}, {{ .Name }}{{ end }}, TIMEOUT{{end}}

#[test]
fn get_card_not_found() {
    let stub = stub::broadcast();
    let controller: u32 = 405419896;
    let card: u32 = 10058401;

    let result = uhppoted::get_card(&stub.cfg, controller, card, TIMEOUT);
    let err = result.as_ref().unwrap_err();

    assert!(err.to_string().contains("Card not found"));
}
