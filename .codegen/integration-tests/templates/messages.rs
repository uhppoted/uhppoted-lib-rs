use std::collections::HashMap;

pub fn message() -> HashMap<[u8; 64], Vec<[u8; 64]>> {
    HashMap::from([
{{- range $test := .Tests }}
        // {{ .Name }}
        (
            [
                {{- range $bytes := .Request }}
                {{ $bytes }}{{ end }}
            ],
            vec![
                {{- range $reply := .Replies }}
                [
                    {{- range $bytes := . }}
                    {{ $bytes }}{{ end }}
                ],{{ end }}
            ],
        ),{{ end }}        
    ])
}
