PORT = 3000

.PHONY: run-api
run-api: stop-api
	@-echo "Running IronShield API inside Docker (in detached mode)..."
	@-echo "Server available at http://localhost:$(PORT)"
	@-echo "Use 'make stop-api' to stop the server."
	@-echo ""
	@docker-compose up -d

.PHONY: rebuild-api
rebuild-api: stop-api
	@-echo "Rebuilding and running IronShield API inside Docker (in detached mode)..."
	@-echo "Server available at http://localhost:$(PORT)"
	@-echo "Use 'make stop-api' to stop the server."
	@-echo ""
	@docker-compose up -d --build

.PHONY: stop-api
stop-api:
	@-echo "Stopping IronShield API container and freeing port $(PORT)..."
	@docker-compose down --remove-orphans || true
	@fuser -k $(PORT)/tcp || true
	@-echo "Stop command finished."
	@-echo ""

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
	     --data '{"endpoint": "https://example.com/protected", "timestamp": '$(shell node -e 'console.log(Date.now())')'}'
	@printf "\n"

.PHONY: check
check:
	@-echo "Running cargo check..."
	cargo check

.PHONY: clean
clean:
	@-echo "Running cargo clean..."
	cargo clean