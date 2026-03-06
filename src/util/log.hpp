#pragma once

#include <filesystem>
#include <format>
#include <fstream>
#include <source_location>

namespace Log {
    inline std::ofstream &GetFile() {
        static std::ofstream file("/tmp/nightshade.log");
        return file;
    }

    inline std::string File(const std::source_location &loc) {
        return std::filesystem::path(loc.file_name()).filename().string();
    }

    template <typename... Args>
    void InfoImpl(const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        GetFile() << std::format(
                         "[INFO] [{}:{}] {}\n", File(loc), loc.line(),
                         std::format(fmt, std::forward<Args>(args)...))
                  << std::flush;
    }

#define Info(...) InfoImpl(std::source_location::current(), __VA_ARGS__)

    template <typename... Args>
    void WarnImpl(const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        GetFile() << std::format(
                         "[WARN] [{}:{}] {}\n", File(loc), loc.line(),
                         std::format(fmt, std::forward<Args>(args)...))
                  << std::flush;
    }

#define Warn(...) WarnImpl(std::source_location::current(), __VA_ARGS__)

    template <typename... Args>
    void ErrorImpl(
        const std::source_location loc, std::format_string<Args...> fmt, Args &&...args) {
        GetFile() << std::format(
                         "[ERROR] [{}:{}] {}\n", File(loc), loc.line(),
                         std::format(fmt, std::forward<Args>(args)...))
                  << std::flush;
    }

#define Error(...) ErrorImpl(std::source_location::current(), __VA_ARGS__)
}  // namespace Log
