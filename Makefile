
cross:
	cross build --release --target x86_64-unknown-linux-gnu

sync:
	rsync -azvhe ssh _site/ fsr:/home/fsr/

deploy:
	scp target/x86_64-unknown-linux-gnu/release/fsr-rust fsr:/home/fsr/

stop:
	ssh -t fsr 'tmux send-keys -t fsr C-c ENTER'

start:
	ssh -t fsr 'tmux send-keys -t fsr ./fsr-rust ENTER'

reload:
	$(MAKE) stop
	$(MAKE) start

full:
	$(MAKE) cross
	$(MAKE) sync
	$(MAKE) stop
	$(MAKE) deploy
	$(MAKE) start
	$(MAKE) reload

help:
	@echo make cross
	@echo make sync
	@echo make stop
	@echo make deploy
	@echo make start
	@echo make reload

.PHONY: cross sync deploy stop start reload full help

