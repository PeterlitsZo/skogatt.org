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
