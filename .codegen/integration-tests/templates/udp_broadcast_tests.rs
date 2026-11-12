use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::ControllerRecord;
use uhppoted::DoorRecord;
use uhppoted::ListenerRecord;
use uhppoted::StatusRecord;

use uhppoted::Alarms;
use uhppoted::Doors;
use uhppoted::Event;

#[path = "stub.rs"]
mod stub;

const TIMEOUT: Duration = Duration::from_millis(5000);
{{ range $test := .Tests }}
{{- range .Annotations}}
#[{{ . }}]{{ end }}
fn {{ .Name }}() {
    let stub = stub::start();

    let expected = {{ .Expected  }};
{{ range $arg := .Args }}
    let {{ $arg.Name }}{{ $arg.Type }} = {{ $arg.Value }};{{ end }}

    let result = uhppoted::{{ .Function }}({{ template "params" .Args }}).unwrap();

    assert_eq!(result, expected);
}
{{ end }}
{{- define "params"}}&stub.cfg{{ range $param := . }}, {{ .Name }}{{ end }}, TIMEOUT{{end}}
