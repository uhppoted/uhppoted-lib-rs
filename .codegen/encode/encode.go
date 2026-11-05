package encode

import (
	_ "embed"
	"fmt"
	"log"
	"net/netip"
	"os"
	"text/template"
	"time"

	lib "codeberg.org/uhppoted/uhppoted-codegen/model/types"

	"codegen/codegen"
	"codegen/model"
)

//go:embed templates/encode_tests.rs
var unitTests string

type test struct {
	Name        string
	Description string
	Function    string
	Args        []arg
	Expected    []string
	Annotations []string
}

type arg struct {
	Name  string
	Type  string
	Value any
	Comma string
}

func UnitTests() {
	log.Println("generating encode unit tests")

	tests := transmogrify(model.Requests)

	if err := os.MkdirAll(".generated/codec", 0750); err != nil {
		log.Fatal(err)
	}

	const file = ".generated/codec/encode_tests.rs"

	if f, err := os.Create(file); err != nil {
		log.Fatalf("%v", err)
	} else {
		defer f.Close()

		encodeTests(f, tests)

		log.Printf("generated %s", file)
	}
}

func encodeTests(f *os.File, tests []test) {
	var data = struct {
		Tests []test
	}{
		Tests: tests,
	}

	tmpl := template.Must(template.New("encode-tests").Funcs(codegen.Functions).Parse(unitTests))
	if err := tmpl.Execute(f, data); err != nil {
		log.Fatalf("%v", err)
	}
}

func transmogrify(requests []lib.Request) []test {
	transmogrified := []test{}

	for _, rq := range requests {
		for _, t := range rq.Tests {
			name := codegen.RustName(t.Name)
			function := codegen.SnakeCase(rq.Name)
			annotations := []string{
				"test",
			}

			if t.Name == "set-IPv4" {
				function = "set_IPv4_request"
				annotations = append(annotations, "allow(non_snake_case)")
			}

			transmogrified = append(transmogrified, test{
				Name:        fmt.Sprintf("%v", name),
				Description: fmt.Sprintf("test encode %v request", rq.Name),
				Function:    function,
				Expected:    packet(t.Expected),
				Args:        args(t),
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

func args(t lib.RequestTest) []arg {
	args := []arg{}

	for ix, a := range t.Args {
		comma := ""
		if ix+1 < len(t.Args) {
			comma = ","
		}

		switch {
		// 		case t.Name == "set-first-card" && a.Name == "active-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		// 		case t.Name == "set-first-card" && a.Name == "inactive-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		case a.Type == "IPv4":
			args = append(args, arg{
				Name:  a.Name,
				Type:  ": Ipv4Addr",
				Value: ipv4(a.Value),
				Comma: comma,
			})

		case a.Type == "datetime":
			args = append(args, arg{
				Name:  a.Name,
				Type:  "",
				Value: datetime(a.Value),
				Comma: comma,
			})

		// 		case a.Type == "date":
		// 			args = append(args, arg{date(a.Value), comma})

		// 		case a.Type == "HHmm":
		// 			args = append(args, arg{HHmm(a.Value), comma})

		// 		case a.Type == "mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		// 		case a.Type == "interlock":
		// 			args = append(args, arg{fmt.Sprintf("To_Interlock (%v)", a.Value), comma})

		// 		case a.Type == "task":
		// 			args = append(args, arg{fmt.Sprintf("To_Task_Type (%v)", a.Value), comma})

		// 		case a.Type == "anti-passback":
		// 			args = append(args, arg{fmt.Sprintf("To_Antipassback (%v)", a.Value), comma})

		// 		case a.Type == "address:port":
		// 			{
		// 				addrPort := netip.MustParseAddrPort(fmt.Sprintf("%v", a.Value))
		// 				address := fmt.Sprintf(`Network_Socket_Address (Addr => Inet_Addr ("%v"), Port => Port_Type (%v))`, addrPort.Addr(), addrPort.Port())

		// 				args = append(args, arg{address, comma})
		// 			}

		// 		case a.Type == "bool":
		// 			args = append(args, arg{codegen.Boolean(a.Value), comma})

		default:
			args = append(args, arg{
				Name:  a.Name,
				Value: fmt.Sprintf("%v", a.Value),
				Comma: comma,
			})
		}
	}

	return args
}

func ipv4(v any) string {
	s := fmt.Sprintf("%v", v)
	addr := netip.MustParseAddr(s)

	return fmt.Sprintf(`"%v".parse().unwrap()`, addr)
}

func datetime(v any) string {
	s := fmt.Sprintf("%v", v)

	if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		panic(fmt.Sprintf("invalid date (%v)", v))
	} else {
		return fmt.Sprintf(`NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap()`, datetime.Format("2006-01-02 15:04:05"))
	}
}

// func date(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if d, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid date (%v)", v))
// 	} else {
// 		year, month, day := d.Date()

// 		return fmt.Sprintf(
// 			"(Year => %v, Month => %v, Day => %v)",
// 			uint16(year), uint8(month), uint8(day))
// 	}
// }

// func HHmm(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if hhmm, err := time.ParseInLocation("15:04", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid HHmm (%v)", v))
// 	} else {
// 		hour := hhmm.Hour()
// 		minute := hhmm.Minute()

// 		return fmt.Sprintf("(Hour => %v, Minute => %v)", uint8(hour), uint8(minute))
// 	}
// }
