#ifndef TransformResult_D_HPP
#define TransformResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum TransformResult {
      TransformResult_Modified = 0,
      TransformResult_Unmodified = 1,
    };
} // namespace capi
} // namespace

class TransformResult {
public:
  enum Value {
    Modified = 0,
    Unmodified = 1,
  };

  TransformResult() = default;
  // Implicit conversions between enum and ::Value
  constexpr TransformResult(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::TransformResult AsFFI() const;
  inline static TransformResult FromFFI(diplomat::capi::TransformResult c_enum);
private:
    Value value;
};


#endif // TransformResult_D_HPP
