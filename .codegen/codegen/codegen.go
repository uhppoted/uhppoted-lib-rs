package codegen

import (
	// "fmt"
	// "net"
	// "net/netip"
	"regexp"
	"strings"
	"text/template"
	// "time"
	// "unicode"
)

var Functions = template.FuncMap{
	// "var":    AdaName,
	// "args":   args,
	// "rpad":   rpad,
	// "record": record,
}

type KV struct {
	Name  string
	Value string
}

func RustName(s string) string {
	tokens := regexp.MustCompile(`[ \-:]+`).Split(s, -1)

	// if s == "set listener addr:port" || s == "set-listener-addr-port" {
	// 	return "Set_Listener"
	// }

	// // 'delay' is a reserved word in Ada
	// if s == "delay" {
	// 	return "Open_Delay"
	// }

	// for i, token := range tokens {
	// 	tokens[i] = capitalize(token)
	// }

	return strings.Join(tokens, "_")
}

// func AdaValue(t string, v any) string {
// 	s := fmt.Sprintf("%v", v)

// 	switch {
// 	case t == "IPv4":
// 		return IPv4(v)

// 	case t == "MAC":
// 		return mac(v)

// 	case t == "version":
// 		return fmt.Sprintf(`To_Unbounded_String ("%v")`, v)

// 	case t == "date" || t == "shortdate" || t == "optional date":
// 		return DateOnly(v)

// 	case t == "time":
// 		return _time(v)

// 	case t == "datetime" || t == "optional datetime":
// 		return datetime(v)

// 	case t == "HHmm":
// 		return HHmm(v)

// 	case t == "bool" && v == true:
// 		return "True"

// 	case t == "bool" && v == false:
// 		return "False"

// 	case t == "Boolean" && v == true:
// 		return "True"

// 	case t == "Boolean" && v == false:
// 		return "False"

// 	case t == "address:port":
// 		{
// 			address := netip.MustParseAddrPort(fmt.Sprintf("%v", v))

// 			return fmt.Sprintf(`Network_Socket_Address (Addr => Inet_Addr ("%v"), Port => Port_Type (%v))`, address.Addr(), address.Port())
// 		}

// 	default:
// 		return s
// 	}
// }

// func KebabCase(s string) string {
// 	tokens := regexp.MustCompile(`\s+`).Split(s, -1)

// 	for i, token := range tokens[1:] {
// 		tokens[i+1] = strings.ToLower(token)
// 	}

// 	return strings.Join(tokens, "-")
// }

func SnakeCase(s string) string {
	tokens := regexp.MustCompile(`\s+`).Split(s, -1)

	for i, token := range tokens[1:] {
		tokens[i+1] = strings.ToLower(token)
	}

	return strings.Join(tokens, "_")
}

// func IPv4(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	addr := netip.MustParseAddr(s).As4()

// 	return fmt.Sprintf("[%v, %v, %v, %v]", addr[0], addr[1], addr[2], addr[3])
// }

// func mac(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if MAC, err := net.ParseMAC(s); err != nil {
// 		panic(fmt.Sprintf("invalid MAC address (%v)", v))
// 	} else {
// 		return fmt.Sprintf("[16#%02x#, 16#%02x#, 16#%02x#, 16#%02x#, 16#%02x#, 16#%02x#]", MAC[0], MAC[1], MAC[2], MAC[3], MAC[4], MAC[5])
// 	}
// }

// func DateOnly(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if date, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid date (%v)", v))
// 	} else {
// 		year, month, day := date.Date()

// 		return fmt.Sprintf("(Year => %v, Month => %v, Day => %v)", uint16(year), uint8(month), uint8(day))
// 	}
// }

// func _time(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if datetime, err := time.ParseInLocation("15:04:05", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid time (%v)", v))
// 	} else {
// 		return fmt.Sprintf(
// 			"(Hour => %v, Minute => %v, Second => %v)",
// 			uint8(datetime.Hour()), uint8(datetime.Minute()), uint8(datetime.Second()))
// 	}
// }

// func datetime(v any) string {
// 	s := fmt.Sprintf("%v", v)

// 	if s == "" {
// 		return fmt.Sprintf("(Year => %v, Month => %v, Day => %v, Hour => %v, Minute => %v, Second => %v)", 1, 1, 1, 0, 0, 0)
// 	} else if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid date (%v)", v))
// 	} else {
// 		year, month, day := datetime.Date()
// 		hour := datetime.Hour()
// 		minute := datetime.Minute()
// 		second := datetime.Second()

// 		return fmt.Sprintf(
// 			"(Year => %v, Month => %v, Day => %v, Hour => %v, Minute => %v, Second => %v)",
// 			uint16(year), uint8(month), uint8(day),
// 			uint8(hour), uint8(minute), uint8(second))
// 	}
// }

// func HHmm(v any) string {
// 	s := fmt.Sprintf("%v", v)
// 	if datetime, err := time.ParseInLocation("15:04", s, time.Local); err != nil {
// 		panic(fmt.Sprintf("invalid HHmm (%v)", v))
// 	} else {
// 		return fmt.Sprintf("(Hour => %v, Minute => %v)", uint8(datetime.Hour()), uint8(datetime.Minute()))
// 	}
// }

// func Boolean(v any) string {
// 	if b, ok := v.(bool); ok {
// 		if b {
// 			return "True"
// 		} else {
// 			return "False"
// 		}
// 	}

// 	if b, ok := v.(string); ok {
// 		if b == "true" {
// 			return "True"
// 		}

// 		if b == "false" {
// 			return "False"
// 		}
// 	}

// 	panic(fmt.Sprintf("invalid boolean (%v)", v))
// }

// func capitalize(s string) string {
// 	runes := []rune(s)

// 	if len(runes) > 0 {
// 		runes[0] = unicode.ToUpper(runes[0])
// 	}

// 	if strings.ToLower(string(runes)) == "ip" {
// 		return "IP"
// 	}

// 	if strings.ToLower(string(runes)) == "ipv4" {
// 		return "IPv4"
// 	}

// 	return string(runes)
// }

// func rpad(length int, s string) string {
// 	if len(s) >= length {
// 		return s
// 	}

// 	return s + strings.Repeat(" ", length-len(s))
// }

// func args(list []any) string {
// 	s := []string{}

// 	for _, v := range list {
// 		s = append(s, fmt.Sprintf("%v", v))
// 	}

// 	return strings.Join(s, ", ")
// }

// func record(fields []KV) string {
// 	s := []string{}
// 	w := 0

// 	for _, v := range fields {
// 		w = max(w, len(v.Name))
// 	}

// 	format := fmt.Sprintf("         %%-%vv => %%v", w)

// 	for _, v := range fields {
// 		s = append(s, fmt.Sprintf(format, v.Name, v.Value))
// 	}

// 	return fmt.Sprintf("(\n%v)", strings.Join(s, ",\n"))
// }
