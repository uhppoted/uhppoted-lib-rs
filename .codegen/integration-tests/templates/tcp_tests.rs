use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::Controller;
use uhppoted::ControllerRecord;
use uhppoted::DoorRecord;
use uhppoted::ListenerRecord;
use uhppoted::StatusRecord;
use uhppoted::Transport;

use uhppoted::Alarms;
use uhppoted::Doors;
use uhppoted::Event;

use crate::stub;

const TIMEOUT: Duration = Duration::from_millis(5000);
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
