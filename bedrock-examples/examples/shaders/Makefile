.SUFFIXES: .vsh .fsh .vspv .fspv

all: triangle.vspv triangle.fspv

.vsh.vspv:
	glslc -o $@ -O -fshader-stage=vert $<

.fsh.fspv:
	glslc -o $@ -O -fshader-stage=frag $<
