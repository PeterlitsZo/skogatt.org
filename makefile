help:
	@echo "[MAKE] Commands:"
	@echo "[MAKE] - help    show this message"
	@echo "[MAKE] - make    make this project to a docker image"
	@echo "[MAKE] - run     run docker container from image"
	@echo "[MAKE] - stop    stop docker container"
	@echo "[MAKE] - rerun   stop and then rerun docker container"
	@echo "[MAKE] - save    save image to a '.tar' file"
	@echo "[MAKE] - shell   access container by shell"
	@echo "[MAKE] - logs    output container's log"

_check_image:
	@if [ "$$(docker images -q peterlitszo/peterlits.com 2> /dev/null)" = "" ]; then \
		echo "\n[MAKE] Cannot find image 'peterlitszo/peterlits.com'"; \
		echo "[MAKE] Please run 'make make' to build one\n"; \
		exit 1; \
	fi

_check_not_running:
	@if [ "$$(docker container inspect -f '{{.State.Status}}' 'peterlits-com' 2> /dev/null)" = "running" ]; then \
		echo "\n[MAKE] 'peterlits-com' is already running"; \
		echo "[MAKE] Run 'make stop' to stop or 'make rerun' to restart\n"; \
		exit 1; \
	fi

_check_running:
	@if [ "$$(docker container inspect -f '{{.State.Status}}' 'peterlits-com' 2> /dev/null)" != "running" ]; then \
		echo "\n[MAKE] docker container 'peterlits-com' is already stop"; \
		echo "[MAKE] Run 'make run' to run it\n"; \
		exit 1; \
	fi

make:
	@echo "\n[MAKE] Building pages file... \n"
	@cd pages && yarn build
	@echo "\n[MAKE] Building server... \n"
	@cd server && make release-build
	@echo "\n[MAKE] Makeing docker image named 'peterlitszo/peterlits.com'... \n"
	@docker build -t peterlitszo/peterlits.com .
	@echo "\n[MAKE] Use 'make run' or 'make rerun' to run docker image\n"

run: _check_image _check_not_running
	@echo "\n[MAKE] Running docker image 'peterlitszo/peterlits.com'...\n"
	@docker run \
		-dp 80:80 \
		--name peterlits-com \
		-v ${PWD}/tmp/:/var/server/ \
		peterlitszo/peterlits.com
	@echo "\n[MAKE] Run 'make stop' to stop running server"
	@echo "[MAKE] Or Access by URL: 'http://localhost'...\n"

stop: _check_running
	@echo "\n[MAKE] Try to remove running container 'peterlits-com'...\n"
	@docker rm -f peterlits-com
	@echo ""

rerun: stop run
	@echo "\n[MAKE] Try to rerunning container 'peterlits-com'...\n"

save: _check_image
	@echo "\n[MAME] Save to peterlits.tar...\n"
	@docker save -o peterlits.tar peterlitszo/peterlits.com
	@echo "\n[MAKE] OK. If want to push file to server, use command:"
	@echo "[MAKE]    'scp peterlits.tar <name>@<IP/domain>:<path>'"
	@echo "[MAKE]    or 'rsync -av peterlits.tar <name>@<IP/domain>:<path>'"
	@echo "[MAKE] Use 'docker load --input <path>/peterlits.tar' to load image\n"

shell: _check_running
	@echo "[MAKE] This command will start a shell...\n"
	@docker exec -it peterlits-com /bin/bash
	@echo ""

logs: _check_running
	@echo "\n[MAKE] logs:\n"
	@docker container logs peterlits-com
	@echo ""
