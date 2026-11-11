use std::net::{Ipv4Addr, SocketAddrV4};
{{ range $test := .Tests }}
    {{- range .Annotations}}
    #[{{ . }}]{{ end }}
    fn {{ .Name }}() {
        let expected: Msg = [
            {{- range $bytes := .Expected }}
            {{ $bytes }}{{ end }}
        ];
        {{ range $arg := .Args }}
        let {{ $arg.Name }}{{ $arg.Type }} = {{ $arg.Value }};{{ end }}

        let result = {{ .Function }}({{- range $arg := .Args }}{{ $arg.Name }}{{ $arg.Comma }}{{ end }}).unwrap();

        assert_eq!(result, expected);
    }
{{ end }}