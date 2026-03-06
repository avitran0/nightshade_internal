#pragma once

inline void **GetVTable(void *gameClass) { return *reinterpret_cast<void ***>(gameClass); }

template <typename Ret, typename... Args>
inline Ret InvokeFunction(void *funcPtr, void *gameClass, Args... args) {
    using FuncPtr = Ret (*)(void *, Args...);
    return reinterpret_cast<FuncPtr>(funcPtr)(gameClass, args...);
}

template <typename Ret, unsigned long Index, typename... Args>
inline Ret Invoke(void *gameClass, Args... args) {
    return InvokeFunction<Ret, Args...>(GetVTable(gameClass)[Index], gameClass, args...);
}

#define VIRTUAL_METHOD(index, name, returnType, argsType, argsCall) \
    inline returnType name argsType { return Invoke<returnType, index> argsCall; }
