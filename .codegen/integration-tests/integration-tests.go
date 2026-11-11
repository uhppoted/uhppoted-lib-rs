package integration_tests

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

//go:embed templates/messages.rs
var messages string

//go:embed templates/udp_broadcast_tests.rs
var udp_broadcast_tests string

type test struct {
	Name     string
	Function string
	Args     []arg
	Request  []string
	Replies  [][]string
	Expected []response
}

type arg struct {
	Name  string
	Type  string
	Value any
}

type response struct {
	Type   string
	Values []value
}

type value struct {
	Name  string
	Type  string
	Value any
}

func IntegrationTests() {
	log.Println("generating integration tests")

	tests := transmogrify(model.API)

	if err := os.MkdirAll(".generated/integration-tests", 0750); err != nil {
		log.Fatal(err)
	}

	// ... stub messages
	{
		const file = ".generated/integration-tests/messages.rs"

		if f, err := os.Create(file); err != nil {
			log.Fatalf("%v", err)
		} else {
			defer f.Close()

			stubMessages(f, tests)

			log.Printf("generated %s", file)
		}
	}

	// ... UDP broadcast tests
	{
		const file = ".generated/integration-tests/udp_broadcast_tests.rs"

		if f, err := os.Create(file); err != nil {
			log.Fatalf("%v", err)
		} else {
			defer f.Close()

			udpBroadcastTests(f, tests)

			log.Printf("generated %s", file)
		}
	}
}

func stubMessages(f *os.File, tests []test) {
	var data = struct {
		Tests []test
	}{
		Tests: tests,
	}

	tmpl := template.Must(template.New("messages").Funcs(codegen.Functions).Parse(messages))
	if err := tmpl.Execute(f, data); err != nil {
		log.Fatalf("%v", err)
	}
}

func udpBroadcastTests(f *os.File, tests []test) {
	var data = struct {
		Tests []test
	}{
		Tests: tests,
	}

	tmpl := template.Must(template.New("udp-broadcast-tests").Funcs(codegen.Functions).Parse(udp_broadcast_tests))
	if err := tmpl.Execute(f, data); err != nil {
		log.Fatalf("%v", err)
	}
}

func transmogrify(functions []lib.Function) []test {
	transmogrified := []test{}

	for _, f := range functions {
		for _, t := range f.Tests {
			name := codegen.SnakeCase(t.Name)

			if t.Name == "set-listener-addr-port" {
				name = "set_listener"
			}

			replies := [][]string{}
			for _, reply := range t.Replies {
				replies = append(replies, packet(reply.Message))
			}

			transmogrified = append(transmogrified, test{
				Name:     name,
				Function: codegen.SnakeCase(f.Name),
				Args:     args(t),
				Request:  packet(t.Request),
				Replies:  replies,
				Expected: expected(f, t),
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

func args(t lib.FuncTest) []arg {
	args := []arg{}

	for _, a := range t.Args {
		switch {
		// 		case t.Name == "set-first-card" && a.Name == "active-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		// 		case t.Name == "set-first-card" && a.Name == "inactive-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		case a.Type == "IPv4":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": Ipv4Addr",
				Value: ipv4(a.Value),
			})

		case a.Type == "address:port":
			{
				addrPort := netip.MustParseAddrPort(fmt.Sprintf("%v", a.Value))
				address := fmt.Sprintf(`"%v".parse().unwrap()`, addrPort)

				args = append(args, arg{
					Name:  codegen.SnakeCase(a.Name),
					Type:  ": SocketAddrV4",
					Value: address,
				})
			}

		case a.Type == "datetime":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  "",
				Value: datetime(a.Value),
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

		// 		case a.Type == "bool":
		// 			args = append(args, arg{codegen.Boolean(a.Value), comma})

		default:
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Value: fmt.Sprintf("%v", a.Value),
			})
		}
	}

	return args
}

func expected(f lib.Function, t lib.FuncTest) []response {
	expected := []response{}

	for _, reply := range t.Replies {
		expected = append(expected, record(f, reply))
	}

	return expected
}

func record(f lib.Function, reply lib.Reply) response {
	lookup := func(key string) any {
		for _, v := range reply.Response {
			if v.Name == key {
				return v.Value
			}
		}

		panic(fmt.Sprintf("missing expected value '%v", key))
	}

	if f.Name == "set listener addr:port" {
		return response{
			Type:   "bool",
			Values: []value{{Name: "", Value: lookup("ok")}},
		}
	}

	if f.Name == "get status" {
		// pub system_time: NaiveDateTime,
		// pub system_error: u8,
		// pub doors: Doors,
		// pub alarms: Alarms,
		// pub event: Event,
		// pub special_info: u8,

		return response{
			Type: "StatusRecord",
			Values: []value{
				{Name: "system_datetime", Value: datetime(lookup("system.datetime"))},
				{Name: "system_error", Value: lookup("system.error")},
				{Name: "system_special_info", Value: lookup("system.special-info")},

				{
					Name: "doors",
					Type: "doors",
					Value: struct {
						Door1Open any
						Door2Open any
						Door3Open any
						Door4Open any

						Door1Button any
						Door2Button any
						Door3Button any
						Door4Button any

						Door1Unlocked any
						Door2Unlocked any
						Door3Unlocked any
						Door4Unlocked any
					}{
						Door1Open: lookup("door-1.open"),
						Door2Open: lookup("door-2.open"),
						Door3Open: lookup("door-3.open"),
						Door4Open: lookup("door-4.open"),

						Door1Button: lookup("door-1.button"),
						Door2Button: lookup("door-2.button"),
						Door3Button: lookup("door-3.button"),
						Door4Button: lookup("door-4.button"),

						Door1Unlocked: lookup("door-1.unlocked"),
						Door2Unlocked: lookup("door-2.unlocked"),
						Door3Unlocked: lookup("door-3.unlocked"),
						Door4Unlocked: lookup("door-4.unlocked"),
					}},

				{
					Name: "alarms",
					Type: "alarms",
					Value: struct {
						Flags      any
						Fire       any
						LockForced any
					}{
						Flags:      lookup("inputs"),
						Fire:       lookup("alarms.fire"),
						LockForced: lookup("alarms.lock-forced"),
					},
				},

				{
					Name: "event",
					Type: "event",
					Value: struct {
						Index         any
						Type          any
						AccessGranted any
						Door          any
						Direction     any
						Card          any
						Timestamp     any
						Reason        any
					}{
						Index:         lookup("event.index"),
						Type:          lookup("event.type"),
						AccessGranted: lookup("event.access-granted"),
						Door:          lookup("event.door"),
						Direction:     lookup("event.direction"),
						Card:          lookup("event.card"),
						Timestamp:     optional_timestamp(lookup("event.timestamp")),
						Reason:        lookup("event.reason"),
					},
				},
			},
		}
	}

	values := []value{}
	for _, v := range reply.Response {
		values = append(values, val(v))
	}

	return response{
		Type:   codegen.TitleCase(fmt.Sprintf("%v", f.Response.Name)),
		Values: values,
	}
}

func val(v lib.Value) value {
	switch {
	// 		case t.Name == "set-first-card" && a.Name == "active-mode":
	// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

	// 		case t.Name == "set-first-card" && a.Name == "inactive-mode":
	// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

	case v.Type == "IPv4":
		return value{
			Name:  codegen.SnakeCase(v.Name),
			Value: ipv4(v.Value),
		}

	case v.Type == "MAC":
		return value{
			Name:  codegen.SnakeCase(v.Name),
			Value: fmt.Sprintf(`"%v"`, v.Value),
		}

	case v.Type == "datetime":
		return value{
			Name:  codegen.SnakeCase(v.Name),
			Value: datetime(v.Value),
		}

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
		return value{
			Name:  codegen.SnakeCase(v.Name),
			Value: fmt.Sprintf("%v", v.Value),
		}
	}
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

func optional_timestamp(v any) string {
	s := fmt.Sprintf("%v", v)

	if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		return "None"
	} else {
		return fmt.Sprintf(`Some(NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap())`, datetime.Format("2006-01-02 15:04:05"))
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
