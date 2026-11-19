package api

import (
	"fmt"
	"time"
)

func MustParseDateOnly(s string) time.Time {
	if s == "" {
		panic(fmt.Errorf("blank date string"))
	} else if date, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
		panic(err)
	} else {
		return date
	}
}

func MustParseDateTime(s string) time.Time {
	if s == "" {
		panic(fmt.Errorf("blank date/time string"))
	} else if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		panic(err)
	} else {
		return datetime
	}
}

func ParseOptionalDate(s string) time.Time {
	if s == "" || s == "0001-01-01" {
		return time.Time{}
	} else if datetime, err := time.ParseInLocation("2006-01-02", s, time.Local); err != nil {
		panic(err)
	} else {
		return datetime
	}
}

func ParseOptionalDateTime(s string) time.Time {
	if s == "" || s == "0001-01-01 00:00:00" {
		return time.Time{}
	} else if datetime, err := time.ParseInLocation("2006-01-02 15:04:05", s, time.Local); err != nil {
		panic(err)
	} else {
		return datetime
	}
}
