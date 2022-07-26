all: rs-lib firmware

rebuild_all: clean rs-lib firmware

rs-lib:
	cd rs_lib/ && cargo build --target thumbv7em-none-eabi --release

clean:
	rm -rf build
	cd rs_lib/ && cargo clean

firmware:
	west build -b nrf52840dk_nrf52840
	cp build/compile_commands.json compile_commands.json

rebuild_firmware:
	west build -b nrf52840dk_nrf52840 --pristine=always
	cp build/compile_commands.json compile_commands.json
