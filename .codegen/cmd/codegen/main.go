package main

import (
	"fmt"
	// "os"
	// "codegen/decode"
	// "codegen/encode"
	// "codegen/integration-tests"
)

var args = struct {
	encode           bool
	decode           bool
	integrationTests bool
}{
	encode:           false,
	decode:           false,
	integrationTests: false,
}

func main() {
	fmt.Printf("\n--- uhppoted-lib-rs::codegen\n\n")

	// if len(os.Args) > 1 {
	// 	for _, arg := range os.Args[1:] {
	// 		switch arg {
	// 		case "--encode":
	// 			args.encode = true

	// 		case "--decode":
	// 			args.decode = true

	// 		case "--integrations-tests":
	// 			args.integrationTests = true

	// 		case "--all":
	// 			args.encode = true
	// 			args.decode = true
	// 			args.integrationTests = true
	// 		}
	// 	}
	// }

	// if args.encode {
	// 	encode.UnitTests()
	// }

	// if args.decode {
	// 	decode.UnitTests()
	// }

	// if args.integrationTests {
	// 	integration_tests.IntegrationTests()
	// }
}
