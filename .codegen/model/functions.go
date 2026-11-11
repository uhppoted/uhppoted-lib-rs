package model

import (
	lib "codeberg.org/uhppoted/uhppoted-codegen/model"

	"codeberg.org/uhppoted/uhppoted-codegen/model/types"
)

var API = []types.Function{
	FindControllers,
	lib.GetController,
	lib.SetIPv4,
	lib.GetTime,
	lib.SetTime,
	lib.GetListener,
	lib.SetListenerAddrPort,
	lib.GetDoor,
	GetStatusRecord,
	// lib.SetDoor,
	// lib.SetDoorPasscodes,
	// lib.OpenDoor,
	// lib.GetCards,
	// lib.GetCard,
	// lib.GetCardAtIndex,
	// lib.PutCard,
	// lib.DeleteCard,
	// lib.DeleteAllCards,
	// lib.GetEvent,
	// lib.GetEventIndex,
	// lib.SetEventIndex,
	// lib.RecordSpecialEvents,
	// lib.GetTimeProfile,
	// lib.SetTimeProfile,
	// lib.ClearTimeProfiles,
	// lib.AddTask,
	// lib.RefreshTaskList,
	// lib.ClearTaskList,
	// lib.SetPCControl,
	// lib.SetInterlock,
	// lib.ActivateKeypads,
	// lib.GetAntiPassback,
	// lib.SetAntiPassback,
	// lib.SetFirstCard,
	// lib.RestoreDefaultParameters,
}

// -- not implemented --
// GetStatus,
// GetListenerAddrPort,
// SetListener,

var FindControllers = types.Function{
	Name: "find-controllers",
	Description: []string{
		"FindControllers retrieves a list of all UHPPOTE controllers accessible via UDP broadcast",
		"on the local LAN.",
	},
	Args:     []types.Arg{},
	Request:  FindControllersRequest.Message,
	Response: FindControllersResponse.Message,

	Tests: []types.FuncTest{
		{
			Name: "find-controllers",
			Args: []types.Arg{},
			Request: []byte{
				0x17, 0x94, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			},
			Replies: []types.Reply{
				{
					Message: []byte{
						0x17, 0x94, 0x00, 0x00, 0x90, 0x53, 0xfb, 0x0b, 0xc0, 0xa8, 0x01, 0x65, 0xff, 0xff, 0xff, 0x00,
						0xc0, 0xa8, 0x01, 0x01, 0x52, 0xfd, 0xfc, 0x07, 0x21, 0x82, 0x06, 0x62, 0x20, 0x20, 0x01, 0x01,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					},
					Response: []types.Value{
						{"controller", "uint32", 201020304},
						{"ip address", "IPv4", "192.168.1.101"},
						{"subnet mask", "IPv4", "255.255.255.0"},
						{"gateway", "IPv4", "192.168.1.1"},
						{"MAC address", "MAC", "52:fd:fc:07:21:82"},
						{"version", "version", "v6.62"},
						{"date", "date", "2020-01-01"},
					},
				},
				{
					Message: []byte{
						0x17, 0x94, 0x00, 0x00, 0x41, 0x78, 0x1e, 0x12, 0xc0, 0xa8, 0x01, 0x64, 0xff, 0xff, 0xff, 0x00,
						0xc0, 0xa8, 0x01, 0x01, 0x52, 0xfd, 0xfc, 0x07, 0x21, 0x82, 0x08, 0x92, 0x20, 0x19, 0x08, 0x15,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					},
					Response: []types.Value{
						{"controller", "uint32", 303986753},
						{"ip address", "IPv4", "192.168.1.100"},
						{"subnet mask", "IPv4", "255.255.255.0"},
						{"gateway", "IPv4", "192.168.1.1"},
						{"MAC address", "MAC", "52:fd:fc:07:21:82"},
						{"version", "version", "v8.92"},
						{"date", "date", "2019-08-15"},
					},
				},
				{
					Message: []byte{
						0x17, 0x94, 0x00, 0x00, 0x78, 0x37, 0x2a, 0x18, 0xc0, 0xa8, 0x01, 0x64, 0xff, 0xff, 0xff, 0x00,
						0xc0, 0xa8, 0x01, 0x01, 0x00, 0x12, 0x23, 0x34, 0x45, 0x56, 0x08, 0x92, 0x20, 0x18, 0x11, 0x05,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					},
					Response: []types.Value{
						{"controller", "uint32", 405419896},
						{"ip address", "IPv4", "192.168.1.100"},
						{"subnet mask", "IPv4", "255.255.255.0"},
						{"gateway", "IPv4", "192.168.1.1"},
						{"MAC address", "MAC", "00:12:23:34:45:56"},
						{"version", "version", "v8.92"},
						{"date", "date", "2018-11-05"},
					},
				},
			},
		},
	},
}

var GetStatusRecord = types.Function{
	Name:        "get status",
	Description: []string{"Retrieves the system status from an access controller."},
	Args: []types.Arg{
		{
			Name:        "controller",
			Type:        "controller",
			Description: "uint32|Controller controller serial number or {id, address, protocol} Controller struct",
		},
	},
	Request:  lib.GetStatusRequest.Message,
	Response: lib.GetStatusResponse.Message,

	Tests: []types.FuncTest{
		{
			Name: "get-status",
			Args: []types.Arg{
				{
					Name:  "controller",
					Type:  "uint32",
					Value: 405419896,
				},
			},
			Request: []byte{
				0x17, 0x20, 0x00, 0x00, 0x78, 0x37, 0x2a, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			},
			Replies: []types.Reply{
				{
					Message: []byte{
						0x17, 0x20, 0x00, 0x00, 0x78, 0x37, 0x2a, 0x18, 0x4e, 0x00, 0x00, 0x00, 0x02, 0x01, 0x03, 0x01,
						0xa1, 0x98, 0x7c, 0x00, 0x20, 0x22, 0x08, 0x23, 0x09, 0x47, 0x06, 0x2c, 0x00, 0x01, 0x00, 0x00,
						0x00, 0x00, 0x00, 0x01, 0x03, 0x09, 0x49, 0x39, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
						0x27, 0x07, 0x09, 0x22, 0x08, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
					},
					Response: []types.Value{
						{
							Name:  "system.datetime",
							Type:  "datetime",
							Value: "2022-08-23 09:49:39",
						},
						{
							Name:  "system.error",
							Type:  "uint8",
							Value: 3,
						},
						{
							Name:  "system.special-info",
							Type:  "uint8",
							Value: 39,
						},

						{
							Name:  "door-1.open",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-1.button",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-1.unlocked",
							Type:  "bool",
							Value: true,
						},

						{
							Name:  "door-2.open",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-2.button",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-2.unlocked",
							Type:  "bool",
							Value: true,
						},

						{
							Name:  "door-3.open",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-3.unlocked",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-3.button",
							Type:  "bool",
							Value: false,
						},

						{
							Name:  "door-4.open",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-4.button",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-4.unlocked",
							Type:  "bool",
							Value: false,
						},

						{
							Name:  "inputs",
							Type:  "uint8",
							Value: 0x09,
						},
						{
							Name:  "alarms.fire",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "alarms.lock-forced",
							Type:  "bool",
							Value: false,
						},

						{
							Name:  "event.index",
							Type:  "uint32",
							Value: 78,
						},
						{
							Name:  "event.type",
							Type:  "event-type",
							Value: 2,
						},
						{
							Name:  "event.access-granted",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "event.door",
							Type:  "uint8",
							Value: 3,
						},
						{
							Name:  "event.direction",
							Type:  "direction",
							Value: 1,
						},
						{
							Name:  "event.card",
							Type:  "uint32",
							Value: 8165537,
						},
						{
							Name:  "event.timestamp",
							Type:  "optional datetime",
							Value: "2022-08-23 09:47:06",
						},
						{
							Name:  "event.reason",
							Type:  "reason",
							Value: 44,
						},
						{
							Name:  "sequence-no",
							Type:  "uint32",
							Value: 0,
						},
					},
				},
			},
		},
		{
			Name: "get-status-no-event",
			Args: []types.Arg{
				{
					Name:  "controller",
					Type:  "uint32",
					Value: 405419897,
				},
			},
			Request: []byte{
				0x17, 0x20, 0x00, 0x00, 0x79, 0x37, 0x2a, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			},
			Replies: []types.Reply{
				{
					Message: []byte{
						0x17, 0x20, 0x00, 0x00, 0x79, 0x37, 0x2a, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
						0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x01,
						0x01, 0x01, 0x00, 0x01, 0x1b, 0x14, 0x37, 0x53, 0xe3, 0x55, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00,
						0x27, 0x07, 0x09, 0x25, 0x11, 0x23, 0x00, 0x00, 0x93, 0x26, 0x04, 0x88, 0x08, 0x92, 0x00, 0x00,
					},
					Response: []types.Value{
						{
							Name:  "controller",
							Type:  "uint32",
							Value: 405419897,
						},
						{
							Name:  "system.datetime",
							Type:  "datetime",
							Value: "2025-11-23 14:37:53",
						},
						{
							Name:  "system.error",
							Type:  "uint8",
							Value: 27,
						},
						{
							Name:  "system.special-info",
							Type:  "uint8",
							Value: 39,
						},

						{
							Name:  "door-1.open",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-1.button",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-1.unlocked",
							Type:  "bool",
							Value: true,
						},

						{
							Name:  "door-2.open",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-2.button",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-2.unlocked",
							Type:  "bool",
							Value: true,
						},

						{
							Name:  "door-3.open",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-3.button",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "door-3.unlocked",
							Type:  "bool",
							Value: true,
						},

						{
							Name:  "door-4.open",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-4.button",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "door-4.unlocked",
							Type:  "bool",
							Value: false,
						},

						{
							Name:  "inputs",
							Type:  "uint8",
							Value: 0x09,
						},
						{
							Name:  "alarms.fire",
							Type:  "bool",
							Value: true,
						},
						{
							Name:  "alarms.lock-forced",
							Type:  "bool",
							Value: false,
						},

						{
							Name:  "event.index",
							Type:  "uint32",
							Value: 0,
						},
						{
							Name:  "event.type",
							Type:  "event-type",
							Value: 0,
						},
						{
							Name:  "event.access-granted",
							Type:  "bool",
							Value: false,
						},
						{
							Name:  "event.door",
							Type:  "uint8",
							Value: 0,
						},
						{
							Name:  "event.direction",
							Type:  "direction",
							Value: 0,
						},
						{
							Name:  "event.card",
							Type:  "uint32",
							Value: 0,
						},
						{
							Name:  "event.timestamp",
							Type:  "optional datetime",
							Value: "",
						},
						{
							Name:  "event.reason",
							Type:  "reason",
							Value: 0,
						},
						{
							Name:  "sequence-no",
							Type:  "uint32",
							Value: 21987,
						},
					},
				},
			},
		},
	},
}
