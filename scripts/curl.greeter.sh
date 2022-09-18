#!/bin/bash
grpcurl -plaintext -import-path ./proto -proto greeter.proto -d '{"name": "Vince"}' '[::]:8080' greeter.Greeter/Greet