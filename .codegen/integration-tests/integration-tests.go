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
	"codegen/model/api"
)

//go:embed templates/messages.rs
var messages string

//go:embed templates/udp_broadcast_tests.rs
var udp_broadcast_tests string

type test struct {
	Name        string
	Function    string
	Args        []arg
	Request     []string
	Replies     [][]string
	Expected    any
	Annotations []string
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
			function := codegen.SnakeCase(f.Name)
			annotations := []string{
				"test",
			}

			if t.Name == "set-listener-addr-port" {
				name = "set_listener"
			}

			if t.Name == "set-IPv4" {
				name = "set_IPv4"
				function = "set_IPv4"
				annotations = append(annotations, "allow(non_snake_case)")
			}

			if f.Name == "set listener addr:port" {
				function = "set_listener"
			}

			println(f.Name)

			replies := [][]string{}
			for _, reply := range t.Replies {
				replies = append(replies, packet(reply.Message))
			}

			transmogrified = append(transmogrified, test{
				Name:        name,
				Function:    function,
				Args:        args(t),
				Request:     packet(t.Request),
				Replies:     replies,
				Expected:    expected(f, t),
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

func args(t lib.FuncTest) []arg {
	args := []arg{}

	for _, a := range t.Args {
		switch {
		case t.Name == "set-listener-addr-port" && a.Name == "interval":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": Option<u8>",
				Value: fmt.Sprintf("Some(%v)", a.Value),
			})

		// 		case t.Name == "set-first-card" && a.Name == "active-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		// 		case t.Name == "set-first-card" && a.Name == "inactive-mode":
		// 			args = append(args, arg{fmt.Sprintf("To_Control_Mode (%v)", a.Value), comma})

		case a.Type == "uint32":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": u32",
				Value: a.Value,
			})

		case a.Type == "uint8":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": u8",
				Value: a.Value,
			})

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
				Type:  ": NaiveDateTime",
				Value: datetime(a.Value),
			})

		case a.Type == "mode":
			args = append(args, arg{
				Name:  codegen.SnakeCase(a.Name),
				Type:  ": u8",
				Value: a.Value,
			})

		// 		case a.Type == "date":
		// 			args = append(args, arg{date(a.Value), comma})

		// 		case a.Type == "HHmm":
		// 			args = append(args, arg{HHmm(a.Value), comma})

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

func expected(f lib.Function, t lib.FuncTest) any {
	lookup := func(reply lib.Reply, key string) any {
		for _, v := range reply.Response {
			if v.Name == key {
				return v.Value
			}
		}

		panic(fmt.Sprintf("missing expected value '%v", key))
	}

	if f.Name == "find-controllers" {
		list := api.ControllerList{}

		for _, reply := range t.Replies {
			list = append(list, api.ControllerRecord{
				Controller: uint32(lookup(reply, "controller").(int)),
				Address:    netip.MustParseAddr(lookup(reply, "ip address").(string)),
				Netmask:    netip.MustParseAddr(lookup(reply, "subnet mask").(string)),
				Gateway:    netip.MustParseAddr(lookup(reply, "gateway").(string)),
				MACAddr:    lookup(reply, "MAC address").(string),
				Version:    lookup(reply, "version").(string),
				Date:       api.MustParseDateOnly(lookup(reply, "date").(string)),
			})
		}

		return list
	}

	if f.Name == "get controller" {
		reply := t.Replies[0]

		return api.ControllerRecord{
			Controller: uint32(lookup(reply, "controller").(int)),
			Address:    netip.MustParseAddr(lookup(reply, "ip address").(string)),
			Netmask:    netip.MustParseAddr(lookup(reply, "subnet mask").(string)),
			Gateway:    netip.MustParseAddr(lookup(reply, "gateway").(string)),
			MACAddr:    lookup(reply, "MAC Address").(string),
			Version:    lookup(reply, "version").(string),
			Date:       api.MustParseDateOnly(lookup(reply, "date").(string)),
		}
	}

	if f.Name == "set IPv4" {
		return lookup(t.Replies[0], "ok").(bool)
	}

	if f.Name == "get time" {
		return fmt.Sprintf(`NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap()`, lookup(t.Replies[0], "date-time").(string))
	}

	if f.Name == "set time" {
		return fmt.Sprintf(`NaiveDateTime::parse_from_str("%v", "%%Y-%%m-%%d %%H:%%M:%%S").unwrap()`, lookup(t.Replies[0], "date-time").(string))
	}

	if f.Name == "get listener" {
		reply := t.Replies[0]

		address := netip.MustParseAddr(lookup(reply, "address").(string))
		port := uint16(lookup(reply, "port").(int))

		return api.ListenerRecord{
			Address:  netip.AddrPortFrom(address, port),
			Interval: uint8(lookup(reply, "interval").(int)),
		}
	}

	if f.Name == "set listener addr:port" {
		return lookup(t.Replies[0], "ok").(bool)
	}

	if f.Name == "get door" {
		reply := t.Replies[0]

		return api.DoorRecord{
			Door:  uint8(lookup(reply, "door").(int)),
			Mode:  uint8(lookup(reply, "mode").(int)),
			Delay: uint8(lookup(reply, "delay").(int)),
		}
	}

	if f.Name == "set door" {
		reply := t.Replies[0]

		return api.DoorRecord{
			Door:  uint8(lookup(reply, "door").(int)),
			Mode:  uint8(lookup(reply, "mode").(int)),
			Delay: uint8(lookup(reply, "delay").(int)),
		}
	}

	if f.Name == "get status" {
		reply := t.Replies[0]

		open := map[uint8]bool{
			1: lookup(reply, "door-1.open").(bool),
			2: lookup(reply, "door-2.open").(bool),
			3: lookup(reply, "door-3.open").(bool),
			4: lookup(reply, "door-4.open").(bool),
		}

		button := map[uint8]bool{
			1: lookup(reply, "door-1.button").(bool),
			2: lookup(reply, "door-2.button").(bool),
			3: lookup(reply, "door-3.button").(bool),
			4: lookup(reply, "door-4.button").(bool),
		}

		unlocked := map[uint8]bool{
			1: lookup(reply, "door-1.unlocked").(bool),
			2: lookup(reply, "door-2.unlocked").(bool),
			3: lookup(reply, "door-3.unlocked").(bool),
			4: lookup(reply, "door-4.unlocked").(bool),
		}

		return api.StatusRecord{
			SystemDateTime:    api.MustParseDateTime((lookup(reply, "system.datetime").(string))),
			SystemError:       uint8(lookup(reply, "system.error").(int)),
			SystemSpecialInfo: uint8(lookup(reply, "system.special-info").(int)),
			Doors: api.Doors{
				Open:     open,
				Button:   button,
				Unlocked: unlocked,
			},
			Alarms: api.Alarms{
				Flags:      uint8(lookup(reply, "inputs").(int)),
				Fire:       lookup(reply, "alarms.fire").(bool),
				LockForced: lookup(reply, "alarms.lock-forced").(bool),
			},
			Event: api.Event{
				Index:         uint32(lookup(reply, "event.index").(int)),
				EventType:     uint8(lookup(reply, "event.type").(int)),
				AccessGranted: lookup(reply, "event.access-granted").(bool),
				Door:          uint8(lookup(reply, "event.door").(int)),
				Direction:     uint8(lookup(reply, "event.direction").(int)),
				Card:          uint32(lookup(reply, "event.card").(int)),
				Timestamp:     api.ParseOptionalDateTime((lookup(reply, "event.timestamp").(string))),
				Reason:        uint8(lookup(reply, "event.reason").(int)),
			},
		}
	}

	fmt.Println(">>>> OOOPS:", f.Name)

	return "???"
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
		return fmt.Sprintf(`"%v".parse().unwrap()`, datetime.Format("2006-01-02T15:04:05"))
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
