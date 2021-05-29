make-pages:
	@echo "\n[MAKE] Building pages file... \n"
	@cd pages && yarn build
	@cd server && make release-build
	@echo "\n[MAKE] Makeing docker image named 'peterlitszo/peterlits.com'... \n"
	@docker build -t peterlitszo/peterlits.com .
	@echo ""

run:
	@echo "\n[MAKE] Before run server, make sure that you have run 'make'..."
	@echo "[MAKE] Running docker image 'peterlitszo/peterlits.com'...\n"
	@docker run -dp 80:80 --name peterlits-com peterlitszo/peterlits.com
	@echo "\n[MAKE] If failed, try to run 'make stop' to stop running server"
	@echo "[MAKE] Or Access by URL: 'http://localhost'...\n"

stop:
	@echo "\n[MAKE] Before stop server, make sure that you have run 'make run'..."
	@echo "[MAKE] Try to remove running server 'peterlitszo/peterlitszo'...\n"
	@docker rm -f peterlits-com
	@echo ""

rerun: stop run

save:
	@echo "\n[MAKE] Before save image, make sure that you have run 'make'..."
	@echo "[MAME] Save to peterlits.tar...\n"
	@docker save -o peterlits.tar peterlitszo/peterlits.com
	@echo "\n[MAKE] OK. If want to push file to server, use command:"
	@echo "[MAKE]    'scp peterlits.tar <name>@<IP/domain>:<path>'"
	@echo "[MAKE] Use 'docker load --input <path>/peterlits.tar' to load image\n"

shell:
	@echo "\n[MAKE] Before save image, make sure that you have run 'make run'..."
	@echo "[MAKE] This command will start a shell...\n"
	@docker exec -it peterlits-com /bin/bash
	@echo ""

logs:
	@echo "\n[MAKE] logs:\n"
	@docker container logs peterlits-com
	@echo ""
