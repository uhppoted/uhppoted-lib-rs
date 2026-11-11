package decode

import (
	_ "embed"
	"fmt"
	"log"
	"net/netip"
	"os"
	// "strings"
	"text/template"
	"time"

	lib "codeberg.org/uhppoted/uhppoted-codegen/model/types"

	"codegen/codegen"
	"codegen/model"
)

//go:embed templates/decode_tests.rs
var unitTests string

type test struct {
	Name        string
	Description string
	Struct      string
	Function    string
	Expected    []val
	Packet      []string
	Annotations []string
}

type val struct {
	Name  string
	Type  string
	Value any
}

func UnitTests() {
	log.Println("generating decode unit tests")

	tests := transmogrify(model.Responses)

	if err := os.MkdirAll(".generated/codec", 0750); err != nil {
		log.Fatal(err)
	}

	const file = ".generated/codec/decode_tests.rs"

	if f, err := os.Create(file); err != nil {
		log.Fatalf("%v", err)
	} else {
		defer f.Close()

		decodeTests(f, tests)

		log.Printf("generated %s", file)
	}
}

func decodeTests(f *os.File, tests []test) {
	var data = struct {
		Tests []test
	}{
		Tests: tests,
	}

	tmpl := template.Must(template.New("decode-tests").Funcs(codegen.Functions).Parse(unitTests))
	if err := tmpl.Execute(f, data); err != nil {
		log.Fatalf("%v", err)
	}
}

func transmogrify(responses []lib.Response) []test {
	transmogrified := []test{}

	for _, response := range responses {
		for _, t := range response.Tests {
			name := codegen.SnakeCase(t.Name)
			function := codegen.SnakeCase(response.Name)
			record := codegen.TitleCase(response.Name)
			annotations := []string{
				"test",
			}

			if t.Name == "set-IPv4" {
				function = "set_IPv4_response"
				annotations = append(annotations, "allow(non_snake_case)")
			}

			transmogrified = append(transmogrified, test{
				Name:        fmt.Sprintf("%v", name),
				Description: fmt.Sprintf("test decode %v request", response.Name),
				Function:    function,
				Packet:      packet(t.Response),
				Struct:      record,
				Expected:    values(t),
				Annotations: annotations,
			})
		}
	}

	return transmogrified
}

func packet(p []byte) []string {
	format := "0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x,  0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x"

	return []string{
		fmt.Sprintf(format, p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15]) + ",",
		fmt.Sprintf(format, p[16], p[17], p[18], p[19], p[20], p[21], p[22], p[23], p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31]) + ",",
		fmt.Sprintf(format, p[32], p[33], p[34], p[35], p[36], p[37], p[38], p[39], p[40], p[41], p[42], p[43], p[44], p[45], p[46], p[47]) + ",",
		fmt.Sprintf(format, p[48], p[49], p[50], p[51], p[52], p[53], p[54], p[55], p[56], p[57], p[58], p[59], p[60], p[61], p[62], p[63]),
	}
}

func values(t lib.ResponseTest) []val {
	values := []val{}

	for _, a := range t.Expected {
		switch {
		case a.Type == "IPv4":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": Ipv4Addr",
				Value: ipv4(a.Value),
			})

		case a.Type == "MAC":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: str(a.Value),
			})

		case a.Type == "version":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: str(a.Value),
			})

		case a.Type == "date":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: dateOnly(a.Value),
			})

		case a.Type == "shortdate":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: shortDate(a.Value),
			})

		case a.Type == "time":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: timeOnly(a.Value),
			})

		case a.Type == "datetime":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: datetime(a.Value),
			})

		case a.Type == "optional datetime":
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: optionalDateTime(a.Value),
			})

		default:
			values = append(values, val{
				Name:  codegen.SnakeCase(a.Name),
				Value: fmt.Sprintf("%v", a.Value),
			})
		}
	}

	return values
}

func ipv4(v any) string {
	s := fmt.Sprintf("%v", v)
	addr := netip.MustParseAddr(s)

	return fmt.Sprintf(`"%v".parse().unwrap()`, addr)
}

func str(v any) string {
	s := fmt.Sprintf("%v", v)

	return fmt.Sprintf(`"%v".to_string()`, s)
}

func dateOnly(v any) string {
	s := fmt.Sprintf("%v", v)

	if date, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid date (%v)", v))
	} else {
		return fmt.Sprintf(`"%v".parse().unwrap()`, date.Format("2006-01-02"))
	}
}

func shortDate(v any) string {
	s := fmt.Sprintf("%v", v)

	if date, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid date (%v)", v))
	} else {
		return fmt.Sprintf(`NaiveDate::parse_from_str("%v", "%%y-%%m-%%d").unwrap()`, date.Format("06-01-02"))
	}
}

func timeOnly(v any) string {
	s := fmt.Sprintf("%v", v)

	if t, err := time.ParseInLocation("15:04:05", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid time (%v)", v))
	} else {
		return fmt.Sprintf(`NaiveTime::parse_from_str("%v", "%%H:%%M:%%S").unwrap()`, t.Format("15:04:05"))
	}
}

func datetime(v any) string {
	s := fmt.Sprintf("%v", v)

	if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid date (%v)", v))
	} else {
		return fmt.Sprintf(`NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap()`, datetime.Format("2006-01-02 15:04:05"))
	}
}

func optionalDateTime(v any) string {
	s := fmt.Sprintf("%v", v)

	if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid timesat (%v)", v))
	} else {
		return fmt.Sprintf(`Some(NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap())`, datetime.Format("2006-01-02 15:04:05"))
	}
}
