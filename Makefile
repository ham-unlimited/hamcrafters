.PHONY: proxy server generate

# Runs the poxy and stores all output (with color) in the file `proxy_output`
proxy:
	script -q -c "cargo run --no-default-features --features proxy" proxy_output

# Just run the server (default features is real server)
server:
	cargo run

# Retrieve current data files from a minecraft server jar, puts a bunch of data files in "generated" folder. 
# Assumes that the current server jar is called "server.jar" and is in the current directory.
generate:
	java -DbundlerMainClass="net.minecraft.data.Main" -jar server.jar -all
