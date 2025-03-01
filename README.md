# MordorWide UDP STUN Relay

This application is used to send UDP packets from a different host in order to detect the NAT type.

## How to Run
### Dev Mode
If the sturnrelay server should be launched in development mode without containerization, do the following steps.
1. Ensure that Rust and Cargo are installed and up to date.
2. Update the environmental variables in `env.standalone` and load it via `ENV_ARGS="$(grep -v '^#' env.standalone  | grep -v '^$' | tr '\n' ' '  )"`.
3. Run `eval "$ENV_ARGS cargo run"` to build and run the sturnrelay server with the environmental variables set.

### Container Mode
1. Make sure that Docker or Podman is installed.
2. Build the image using `docker compose -f docker-compose.standalone.yml build`.
3. Update the environmental variables in `env.standalone`.
4. Run the standalone container with `docker compose --env-file env.standalone -f docker-compose.standalone.yml up -d`
5. Check the logs via `docker logs -f mordorwide-stunrelay`
6. Stop the standalone server again with `docker compose --env-file env.standalone -f docker-compose.standalone.yml down -v`

## Client Testing
The STUN connection can be tested in one direction as follows.
```
# Make sure that the STUNRelay server is running...

# Run the receiving UDP endpoint at port 9999 in a second terminal window.
nc -ul 9999

# Configure the STURNRelay server to relay port 8888 to 9999 (and vice versa)
curl -X POST -H "Content-Type: application/json" \
    --data '{"client_ip": "127.0.0.1", "client_port": 9999, "source_port": 8888, "b64_payload": "'$(echo "Test Message Base64" | base64 )'"}' \
    http://localhost:8080/send

# The result should look like this:
# $> {"success":true}

# The netcat receiver endpoint should show "Test Message Base64" in the second terminal window.
```