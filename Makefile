
cross:
	cross build --release --target x86_64-unknown-linux-gnu

sync:
	rsync -azvhe ssh _site/ fsr:/home/fsr/

.PHONY: cross sync
