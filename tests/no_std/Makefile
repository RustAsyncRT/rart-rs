all: lib build_sample run
rebuilt_all: clean lib build_sample run

lib:
	cd rs_lib/ && cargo build --target thumbv7em-none-eabi --release

clean:
	rm -rf build
	cd rs_lib/ && cargo clean

build_sample:
	west build -b nrf52840dk_nrf52840
	cp build/compile_commands.json compile_commands.json

rebuild_sample:
	west build -b nrf52840dk_nrf52840 --pristine=always
	cp build/compile_commands.json compile_commands.json

run:
	renode 3rdparty/renode/setup_nrf52840.resc
