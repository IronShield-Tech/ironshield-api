PORT = 3000

.PHONY: run-api
run-api:
	@-echo "Running IronShield API inside Docker..."
	@-echo "Server available at http://localhost:$(PORT)"
	@-echo "CTRL+C to stop the server"
	@-echo ""
	@docker-compose up -d --build

.PHONY: stop-api
stop-api:
	@-echo "Stopping IronShield API container..."
	@-echo ""
	@docker-compose down

.PHONY: test-api
test-api:
	@-echo "Running IronShield API tests..."
	@if ! curl -s http://localhost:$(PORT) > /dev/null; then \
      echo "API Server is NOT running. Start it with: 'make run-api'"; \
      exit 1; \
    fi
    # TODO: Add tests.
    
.PHONY: test-request
test-request:
	@curl --request POST http://localhost:3000/request \
	     --header "Content-Type: application/json"     \
	     --data '{"endpoint": "https://example.com/protected", "timestamp": '$(shell date +%s%3N)'}'
	@printf "\n"

.PHONY: check
check:
	@-echo "Running cargo check..."
	cargo check

.PHONY: clean
clean:
	@-echo "Running cargo clean..."
	cargo clean