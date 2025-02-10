all: 

cargo: 	
	cargo watch -x run 

build: 
	cargo build


# buildar em release para diminuir o tamanho do binário do arquivo.
release:	
	cargo build --release 