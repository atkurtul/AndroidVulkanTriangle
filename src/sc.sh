glslc -c shader.frag -o frag.spv
glslc -c shader.vert -o vert.spv
hexdump -v -e '15/1 "0x%02x, "1/1 " 0x%02x,\n"' "frag.spv" > frag.hex
hexdump -v -e '15/1 "0x%02x, "1/1 " 0x%02x,\n"' "vert.spv" > vert.hex
#rm frag.spv vert.spv