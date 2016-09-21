PROGRAM=hello-embed-rs
ELF=target/thumbv7em-none-eabi/release/${PROGRAM}
OBJCOPY=arm-none-eabi-objcopy
BIN=${PROGRAM}.bin

all: ${BIN}

${BIN}: ${ELF}
	${OBJCOPY} -Obinary $< $@

$(ELF):
	xargo build --release

upload: ${BIN}
	st-flash --reset write $< 0x8000000

clean:
	xargo clean

.PHONY: ${ELF} $(BIN) clean all
