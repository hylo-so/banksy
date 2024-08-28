a quick-pre readme for banksy until I get a real readme up.

install : pnpm install
build: pnpm build
after-build: pnpm tsc, then copy the .node file inside of the banksy subfolder into the dist folder.
inside of project using banksy for testing(protocol) type: pnpm link ../banksy
