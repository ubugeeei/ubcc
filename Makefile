image:
	- docker build -t compilerbook https://www.sigbus.info/compilerbook/Dockerfile

login:
	- docker run --rm -it -v $(CURDIR):/ws compilerbook

build:
	- cross build --target x86_64-unknown-linux-musl

test:
	- cargo test

e2e:
	- docker run -it -v $(CURDIR):/ws -w /ws compilerbook ./test.sh
