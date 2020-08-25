#include <shaderc/shaderc.h>

struct Result {
  shaderc_compiler_t compiler;
  shaderc_compilation_result_t result;
  const char *src;
  size_t len;
};

struct Result compile(const char *src, size_t len)
{
  struct Result res =  {shaderc_compiler_initialize()};
  res.result = shaderc_compile_into_spv(res.compiler, src, len, shaderc_vertex_shader, "VERTEXSHADER", "main", 0);
  res.src = shaderc_result_get_bytes(res.result);
  res.len = shaderc_result_get_length(res.result);
  return res;
}