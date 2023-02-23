#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

const char *hello_world();

/// # Not Safety
/// 这个函数是不安全的，别调！
const char *hello_bad(const char *name);

const char *hello(const char *name);

/// 要习惯这样的“释放内存”的写法，因为它实际上借助了 Rust 的所有权规则：
/// 当所有者离开作用域时，拥有的内存会被释放。
/// 这里我们创建一个有所有权的对象，就是为了函数结束时的自动释放。
void free_str(char *s);

} // extern "C"
