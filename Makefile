
cross:
	cross build --release --target x86_64-unknown-linux-gnu

pull-content:
	rsync -azvhe ssh fsr:/home/fsr/content/ _site/content/ 

push-templates:
	rsync -azvhe ssh _site/templates/ fsr:/home/fsr/templates/
	rsync -azvhe ssh _site/static/ fsr:/home/fsr/static/

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
	$(MAKE) stop
	$(MAKE) deploy
	$(MAKE) start
	$(MAKE) reload

help:
	@echo make cross
	@echo make pull-content
	@echo make push-templates
	@echo make stop
	@echo make deploy
	@echo make start
	@echo make reload

.PHONY: cross pull-content push-templates deploy stop start reload full help

