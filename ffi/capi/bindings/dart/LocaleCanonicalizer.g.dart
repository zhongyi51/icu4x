// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// A locale canonicalizer.
///
/// See the [Rust documentation for `LocaleCanonicalizer`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html) for more information.
final class LocaleCanonicalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  LocaleCanonicalizer._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XLocaleCanonicalizer_destroy));

  /// Create a new [`LocaleCanonicalizer`].
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.new) for more information.
  ///
  /// Throws [Error] on failure.
  factory LocaleCanonicalizer(DataProvider provider) {
    final result = _ICU4XLocaleCanonicalizer_create(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleCanonicalizer._(result.union.ok, true);
  }

  /// Create a new [`LocaleCanonicalizer`] with extended data.
  ///
  /// See the [Rust documentation for `new_with_expander`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.new_with_expander) for more information.
  ///
  /// Throws [Error] on failure.
  factory LocaleCanonicalizer.extended(DataProvider provider) {
    final result = _ICU4XLocaleCanonicalizer_create_extended(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return LocaleCanonicalizer._(result.union.ok, true);
  }

  /// FFI version of `LocaleCanonicalizer::canonicalize()`.
  ///
  /// See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.canonicalize) for more information.
  TransformResult canonicalize(Locale locale) {
    final result = _ICU4XLocaleCanonicalizer_canonicalize(_underlying, locale._underlying);
    return TransformResult.values[result];
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XLocaleCanonicalizer_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XLocaleCanonicalizer_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XLocaleCanonicalizer_create')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XLocaleCanonicalizer_create(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XLocaleCanonicalizer_create_extended')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XLocaleCanonicalizer_create_extended(ffi.Pointer<ffi.Opaque> provider);

@ffi.Native<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XLocaleCanonicalizer_canonicalize')
// ignore: non_constant_identifier_names
external int _ICU4XLocaleCanonicalizer_canonicalize(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> locale);
