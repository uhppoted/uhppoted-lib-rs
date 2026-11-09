use chrono::NaiveDateTime;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;

use uhppoted::ControllerRecord;
use uhppoted::ListenerRecord;

#[path = "stub.rs"]
mod stub;

const TIMEOUT: Duration = Duration::from_millis(1250);

{{- range $test := .Tests }}
#[test]
fn {{ .Name }}() {
    let stub = stub::start();
    {{ if gt (len .Replies) 1}}
    let expected = vec![{{ range $reply := .Expected}}
        {{ template "response" . }},{{ end }}
    ];{{ else }}
    let expected = {{ template "response" (index .Expected  0) }};{{ end }}
    {{ range $arg := .Args }}
    {{ .Name }}{{ .Type }} = {{ .Value }};{{ end }}

    let result = uhppoted::{{ .Function }}({{ template "params" .Args }}).unwrap();

    assert_eq!(result, expected);
}
{{ end }}

{{- define "params"}}&stub.cfg{{ range $param := . }}, {{ .Name }}{{ end }}, TIMEOUT{{end}}

{{- define "response"}}{{ .Type }} { {{ range $value := .Values }}
        {{ .Name }}: {{ .Value }},{{ end }}
    }{{ end }}