setup4:
	sudo ./netns.sh

setup6:
	sudo ./netns_ipv6.sh

run4:
	cargo build --example main
	sudo ip netns exec host2 ./target/debug/examples/main

run6:
	cargo build --example main
	sudo ip netns exec host2 ./target/debug/examples/main
