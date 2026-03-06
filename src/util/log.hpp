#pragma once

#include <format>
#include <fstream>
#include <source_location>

namespace log {
    extern std::ofstream file;

    template <typename... Args>
    void InfoImpl(const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        file << std::format(
            "[INFO] {} at [{}:{} in {}]", std::format(fmt, std::forward<Args>(args)...),
            loc.file_name(), loc.line(), loc.function_name());
    }

#define Info(...) InfoImpl(std::source_location::current(), __VA_ARGS__)

    template <typename... Args>
    void WarnImpl(const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        file << std::format(
            "[WARN] {} at [{}:{} in {}]", std::format(fmt, std::forward<Args>(args)...),
            loc.file_name(), loc.line(), loc.function_name());
    }

#define Warn(...) WarnImpl(std::source_location::current(), __VA_ARGS__)

    template <typename... Args>
    void ErrorImpl(
        const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        file << std::format(
            "[ERROR] {} at [{}:{} in {}]", std::format(fmt, std::forward<Args>(args)...),
            loc.file_name(), loc.line(), loc.function_name());
    }

#define Error(...) ErrorImpl(std::source_location::current(), __VA_ARGS__)
}  // namespace log
