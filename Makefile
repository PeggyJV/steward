.DEFAULT_GOAL := e2e_rebalance

VALIDATOR_IMAGE := "ghcr.io/peggyjv/sommelier-sommelier:main"
ORCHESTRATOR_IMAGE := "ghcr.io/peggyjv/gravity-bridge-orchestrator:main"

e2e_build_images: e2e_clean_slate
	@docker pull $(VALIDATOR_IMAGE)
	@docker tag $(VALIDATOR_IMAGE) sommelier:prebuilt
	@docker pull $(ORCHESTRATOR_IMAGE)
	@docker tag $(ORCHESTRATOR_IMAGE) orchestrator:prebuilt
	@docker build -t steward:prebuilt -f Dockerfile .
	@docker build -t ethereum:prebuilt -f integration_tests/ethereum/Dockerfile integration_tests/ethereum/

e2e_clean_slate:
	@docker rm --force \
		$(shell docker ps -qa --filter="name=ethereum") \
		$(shell docker ps -qa --filter="name=sommelier") \
		$(shell docker ps -qa --filter="name=orchestrator") \
		$(shell docker ps -qa --filter="name=steward") \
		1>/dev/null \
		2>/dev/null \
		|| true
	@docker wait \
		$(shell docker ps -qa --filter="name=ethereum") \
		$(shell docker ps -qa --filter="name=sommelier") \
		$(shell docker ps -qa --filter="name=orchestrator") \
		$(shell docker ps -qa --filter="name=steward") \
		1>/dev/null \
		2>/dev/null \
		|| true
	@docker network prune --force 1>/dev/null 2>/dev/null || true
	@cd integration_tests && go test -c

e2e_cork_test: e2e_clean_slate
	@E2E_SKIP_CLEANUP=true integration_tests/integration_tests.test -test.failfast -test.v -test.run IntegrationTestSuite -testify.m TestCork || make -s fail

fail:
	@echo 'test failed; dumping container logs into ./testdata for review'
	@mkdir -p ./testdata
	@docker logs ethereum > testdata/ethereum.log 2>&1 || true
	@docker logs sommelier0 > testdata/sommelier0.log 2>&1 || true
	@docker logs sommelier1 > testdata/sommelier1.log 2>&1 || true
	@docker logs sommelier2 > testdata/sommelier2.log 2>&1 || true
	@docker logs sommelier3 > testdata/sommelier3.log 2>&1 || true
	@docker logs orchestrator0 > testdata/orchestrator0.log 2>&1 || true
	@docker logs orchestrator1 > testdata/orchestrator1.log 2>&1 || true
	@docker logs orchestrator2 > testdata/orchestrator2.log 2>&1 || true
	@docker logs orchestrator3 > testdata/orchestrator3.log 2>&1 || true
	@docker logs steward0 > testdata/steward0.log 2>&1 || true
	@docker logs steward1 > testdata/steward1.log 2>&1 || true
	@docker logs steward2 > testdata/steward2.log 2>&1 || true
	@docker logs steward3 > testdata/steward3.log 2>&1 || true
	@false

