// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// See the [Rust documentation for `Collator`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html) for more information.
final class Collator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Collator._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XCollator_destroy));

  /// Construct a new Collator instance.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory Collator(DataProvider provider, Locale locale, CollatorOptions options) {
    final temp = ffi2.Arena();
    final result = _ICU4XCollator_create_v1(provider._underlying, locale._underlying, options._pointer(temp));
    temp.releaseAll();
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return Collator._(result.union.ok, true);
  }

  /// Compare two strings.
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `compare_utf16`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf16) for more information.
  Ordering compare(String left, String right) {
    final temp = ffi2.Arena();
    final leftView = left.utf16View;
    final rightView = right.utf16View;
    final result = _ICU4XCollator_compare_utf16(_underlying, leftView.pointer(temp), leftView.length, rightView.pointer(temp), rightView.length);
    temp.releaseAll();
    return Ordering.values.firstWhere((v) => v._underlying == result);
  }

  /// The resolved options showing how the default options, the requested options,
  /// and the options from locale data were combined. None of the struct fields
  /// will have `Auto` as the value.
  ///
  /// See the [Rust documentation for `resolved_options`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.resolved_options) for more information.
  ResolvedCollatorOptions get resolvedOptions {
    final result = _ICU4XCollator_resolved_options(_underlying);
    return ResolvedCollatorOptions._(result);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XCollator_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XCollator_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _CollatorOptionsFfi)>(isLeaf: true, symbol: 'ICU4XCollator_create_v1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCollator_create_v1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, _CollatorOptionsFfi options);

@ffi.Native<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint16>, ffi.Size, ffi.Pointer<ffi.Uint16>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCollator_compare_utf16')
// ignore: non_constant_identifier_names
external int _ICU4XCollator_compare_utf16(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint16> leftData, int leftLength, ffi.Pointer<ffi.Uint16> rightData, int rightLength);

@ffi.Native<_ResolvedCollatorOptionsFfi Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCollator_resolved_options')
// ignore: non_constant_identifier_names
external _ResolvedCollatorOptionsFfi _ICU4XCollator_resolved_options(ffi.Pointer<ffi.Opaque> self);
