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

#[path = "../stub.rs"]
mod stub;

const TIMEOUT: Duration = Duration::from_millis(5000);

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
    let {{ .Name }}{{ .Type }} = {{ .Value }};{{ end }}

    let result = uhppoted::{{ .Function }}({{ template "params" .Args }}).unwrap();

    assert_eq!(result, expected);
}
{{ end }}

{{- define "params"}}&stub.cfg{{ range $param := . }}, {{ .Name }}{{ end }}, TIMEOUT{{end}}

{{- define "response"}}
        {{- if eq .Type "bool"}}{{ (index .Values 0).Value }}
        {{- else }}
        {{- .Type }} { {{ range $value := .Values }}
        {{ if eq .Type "doors"}}{{- .Name }}: {{template "doors" .Value }}
        {{- else if eq .Type "alarms"}}{{- .Name }}: {{template "alarms" .Value }}
        {{- else if eq .Type "event"}}{{- .Name }}: {{template "event" .Value }}
        {{- else}}{{- .Name }}: {{ .Value }},{{ end }}{{ end }}}{{ end }}{{ end }}

{{- define "doors"}}Doors {
            open: HashMap::from([
                (1, {{ .Door1Open }}),
                (2, {{ .Door2Open }}),
                (3, {{ .Door3Open }}),
                (4, {{ .Door4Open }}),
            ]),

            button: HashMap::from([
                (1, {{ .Door1Button }}),
                (2, {{ .Door2Button }}),
                (3, {{ .Door3Button }}),
                (4, {{ .Door4Button }}),
            ]),

            unlocked: HashMap::from([
                (1, {{ .Door1Unlocked }}),
                (2, {{ .Door2Unlocked }}),
                (3, {{ .Door3Unlocked }}),
                (4, {{ .Door4Unlocked }}),
            ]),
        },{{ end }}

{{- define "alarms"}}Alarms {
            flags: {{ .Flags }},
            fire: {{ .Fire }},
            lock_forced: {{ .LockForced  }},
        },{{ end }}

{{- define "event"}}Some(Event {
            index: {{ .Index }},
            event_type: {{ .Type }},
            access_granted: {{ .AccessGranted }},
            door: {{ .Door }},
            direction: {{ .Direction }},
            card: {{ .Card }},
            timestamp: {{ .Timestamp }},
            reason: {{ .Reason }},
        }),{{ end }}
