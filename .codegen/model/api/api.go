package api

import (
	"fmt"
	"net/netip"
	"strings"
	"time"
)

type ControllerList []ControllerRecord

type ControllerRecord struct {
	Controller uint32
	Address    netip.Addr
	Netmask    netip.Addr
	Gateway    netip.Addr
	MACAddr    string
	Version    string
	Date       time.Time
}

type ListenerRecord struct {
	Address  netip.AddrPort
	Interval uint8
}

type DoorRecord struct {
	Door  uint8
	Mode  uint8
	Delay uint8
}

type StatusRecord struct {
	SystemDateTime    time.Time
	SystemError       uint8
	SystemSpecialInfo uint8
	Doors             Doors
	Alarms            Alarms
	Event             Event
}

type Doors struct {
	Open     map[uint8]bool
	Button   map[uint8]bool
	Unlocked map[uint8]bool
}

type Alarms struct {
	Flags      uint8
	Fire       bool
	LockForced bool
}

type Event struct {
	Index         uint32
	EventType     uint8
	AccessGranted bool
	Door          uint8
	Direction     uint8
	Card          uint32
	Timestamp     time.Time
	Reason        uint8
}

func (v ControllerList) String() string {
	var b strings.Builder

	fmt.Fprintf(&b, "vec![\n")
	for _, e := range v {
		fmt.Fprintf(&b, "        %s {\n", "ControllerRecord")
		fmt.Fprintf(&b, "            controller: %v,\n", e.Controller)
		fmt.Fprintf(&b, "            ip_address: \"%v\".parse().unwrap(),\n", e.Address)
		fmt.Fprintf(&b, "            subnet_mask: \"%v\".parse().unwrap(),\n", e.Netmask)
		fmt.Fprintf(&b, "            gateway: \"%v\".parse().unwrap(),\n", e.Gateway)
		fmt.Fprintf(&b, "            MAC_address: \"%v\".to_string(),\n", e.MACAddr)
		fmt.Fprintf(&b, "            version: \"%v\".to_string(),\n", e.Version)
		fmt.Fprintf(&b, "            date: \"%v\".parse().unwrap(),\n", e.Date.Format("2006-01-02"))
		fmt.Fprintf(&b, "        },\n")
	}

	fmt.Fprintf(&b, "    ]")

	return b.String()
}

func (v ControllerRecord) String() string {
	var b strings.Builder

	fmt.Fprintf(&b, "%s {\n", "ControllerRecord")
	fmt.Fprintf(&b, "        controller: %v,\n", v.Controller)
	fmt.Fprintf(&b, "        ip_address: \"%v\".parse().unwrap(),\n", v.Address)
	fmt.Fprintf(&b, "        subnet_mask: \"%v\".parse().unwrap(),\n", v.Netmask)
	fmt.Fprintf(&b, "        gateway: \"%v\".parse().unwrap(),\n", v.Gateway)
	fmt.Fprintf(&b, "        MAC_address: \"%v\".to_string(),\n", v.MACAddr)
	fmt.Fprintf(&b, "        version: \"%v\".to_string(),\n", v.Version)
	fmt.Fprintf(&b, "        date: \"%v\".parse().unwrap(),\n", v.Date.Format("2006-01-02"))
	fmt.Fprintf(&b, "    }")

	return b.String()
}

func (v ListenerRecord) String() string {
	var b strings.Builder

	fmt.Fprintf(&b, "%s {\n", "ListenerRecord")
	fmt.Fprintf(&b, "        address: \"%v\".parse().unwrap(),\n", v.Address)
	fmt.Fprintf(&b, "        interval: %v,\n", v.Interval)
	fmt.Fprintf(&b, "    }")

	return b.String()
}

func (v DoorRecord) String() string {
	var b strings.Builder

	fmt.Fprintf(&b, "%s {\n", "DoorRecord")
	fmt.Fprintf(&b, "        door: %v,\n", v.Door)
	fmt.Fprintf(&b, "        mode: %v,\n", v.Mode)
	fmt.Fprintf(&b, "        delay: %v,\n", v.Delay)
	fmt.Fprintf(&b, "    }")

	return b.String()
}

func (v StatusRecord) String() string {
	var b strings.Builder

	fmt.Fprintf(&b, "%s {\n", "StatusRecord")
	fmt.Fprintf(&b, "        system_datetime: NaiveDateTime::parse_from_str(\"%v\", \"%%Y-%%m-%%d %%H:%%M:%%S\").unwrap(),\n", v.SystemDateTime.Format("2006-01-02 15:04:05"))
	fmt.Fprintf(&b, "        system_error: %v,\n", v.SystemError)
	fmt.Fprintf(&b, "        system_special_info: %v,\n", v.SystemSpecialInfo)
	fmt.Fprintf(&b, "\n")
	fmt.Fprintf(&b, "        doors: Doors {\n")
	fmt.Fprintf(&b, "            open: HashMap::from([(1, %v), (2, %v), (3, %v), (4, %v)]),\n", v.Doors.Open[1], v.Doors.Open[2], v.Doors.Open[3], v.Doors.Open[4])
	fmt.Fprintf(&b, "            button: HashMap::from([(1, %v), (2, %v), (3, %v), (4, %v)]),\n", v.Doors.Button[1], v.Doors.Button[2], v.Doors.Button[3], v.Doors.Button[4])
	fmt.Fprintf(&b, "            unlocked: HashMap::from([(1, %v), (2, %v), (3, %v), (4, %v)]),\n", v.Doors.Unlocked[1], v.Doors.Unlocked[2], v.Doors.Unlocked[3], v.Doors.Unlocked[4])
	fmt.Fprintf(&b, "        },\n")
	fmt.Fprintf(&b, "\n")
	fmt.Fprintf(&b, "        alarms: Alarms {\n")
	fmt.Fprintf(&b, "            flags: %v,\n", v.Alarms.Flags)
	fmt.Fprintf(&b, "            fire: %v,\n", v.Alarms.Fire)
	fmt.Fprintf(&b, "            lock_forced: %v,\n", v.Alarms.LockForced)
	fmt.Fprintf(&b, "        },\n")
	fmt.Fprintf(&b, "\n")

	if v.Event.Index == 0 {
		fmt.Fprintf(&b, "        event: None,\n")

	} else {
		fmt.Fprintf(&b, "        event: Some(Event {\n")
		fmt.Fprintf(&b, "            index: %v,\n", v.Event.Index)
		fmt.Fprintf(&b, "            event_type: %v,\n", v.Event.EventType)
		fmt.Fprintf(&b, "            access_granted: %v,\n", v.Event.AccessGranted)
		fmt.Fprintf(&b, "            door: %v,\n", v.Event.Door)
		fmt.Fprintf(&b, "            direction: %v,\n", v.Event.Direction)
		fmt.Fprintf(&b, "            card: %v,\n", v.Event.Card)
		fmt.Fprintf(&b, "            timestamp: Some(NaiveDateTime::parse_from_str(\"%v\", \"%%Y-%%m-%%d %%H:%%M:%%S\").unwrap()),\n", v.Event.Timestamp.Format("2006-01-02 15:04:05"))
		fmt.Fprintf(&b, "            reason: %v,\n", v.Event.Reason)
		fmt.Fprintf(&b, "        }),\n")
	}
	fmt.Fprintf(&b, "    }")

	return b.String()
}
