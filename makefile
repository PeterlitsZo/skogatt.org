make-pages:
	@echo "\n[MAKE] Building pages file... \n"
	@cd pages && yarn build
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
	@echo "\n[MAKE] Try to remove running server 'peterlitszo/peterlitszo'...\n"
	@docker rm -f peterlits-com
	@echo ""

save:
	@echo "\n[MAKE] Before save image, make sure that you have run 'make'..."
	@echo "[MAME] Save to peterlits.tar...\n"
	@docker save -o peterlits.tar peterlitszo/peterlits.com
	@echo "\n[MAKE] OK. If want to push file to server, use command:"
	@echo "[MAKE]    'scp peterlits.tar <name>@<IP/domain>:<path>'"
	@echo "[MAKE] Use 'docker load --input <path>/peterlits.tar' to load image\n"
