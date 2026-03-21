.PHONY: proxy server generate clean_generate

# Runs the poxy and stores all output (with color) in the file `proxy_output`
proxy:
	script -q -c "cargo run --no-default-features --features proxy" proxy_output

# Just run the server (default features is real server)
server:
	cargo run

clean_generate:
	rm -rf generated TMP_GENERATE

# Retrieve current data files from a minecraft server jar, puts a bunch of data files in "generated" folder. 
# Assumes that the current server jar is called "server.jar" and is in the current directory.
generate: server.jar clean_generate
	mkdir TMP_GENERATE
	cp server.jar TMP_GENERATE/server.jar
	cd TMP_GENERATE && java -DbundlerMainClass="net.minecraft.data.Main" -jar server.jar -all
	mv TMP_GENERATE/generated ./
	rm -rf TMP_GENERATE
