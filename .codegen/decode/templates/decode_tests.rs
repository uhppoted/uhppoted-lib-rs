{{ range $test := .Tests }}
    {{- range .Annotations}}
    #[{{ . }}]{{ end }}
    fn {{ .Name }}() {
        let packet: Msg = [
            {{- range $bytes := .Packet }}
            {{ $bytes }}{{ end }}
        ];

        let expected = {{ .Struct }} {
            {{- range $val := .Expected }}
            {{ .Name }}: {{ .Value }},{{ end }}
        };

        let response = {{ .Function }}(&packet).unwrap();

        assert_eq!(response, expected);        
    }
{{ end }}