
cross:
	cross build --release --target x86_64-unknown-linux-gnu

sync:
	rsync -azvhe ssh _site/ fsr:/home/fsr/

deploy:
	scp target/x86_64-unknown-linux-gnu/release/fsr-rust fsr:/home/fsr/

reload:
	ssh -t fsr 'tmux send-keys -t fsr C-c ENTER ./fsr-rust ENTER'

.PHONY: cross sync deploy reload

