// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// An object containing bidi information for a given string, produced by `for_text()` on `Bidi`
///
/// See the [Rust documentation for `BidiInfo`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html) for more information.
final class BidiInfo implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  BidiInfo._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XBidiInfo_destroy));

  /// The number of paragraphs contained here
  int get paragraphCount {
    final result = _ICU4XBidiInfo_paragraph_count(_underlying);
    return result;
  }

  /// Get the nth paragraph, returning `None` if out of bounds
  BidiParagraph? paragraphAt(int n) {
    final result = _ICU4XBidiInfo_paragraph_at(_underlying, n);
    return result.address == 0 ? null : BidiParagraph._(result, true);
  }

  /// The number of bytes in this full text
  int get size {
    final result = _ICU4XBidiInfo_size(_underlying);
    return result;
  }

  /// Get the BIDI level at a particular byte index in the full text.
  /// This integer is conceptually a `unicode_bidi::Level`,
  /// and can be further inspected using the static methods on ICU4XBidi.
  ///
  /// Returns 0 (equivalent to `Level::ltr()`) on error
  int levelAt(int pos) {
    final result = _ICU4XBidiInfo_level_at(_underlying, pos);
    return result;
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XBidiInfo_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XBidiInfo_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XBidiInfo_paragraph_count')
// ignore: non_constant_identifier_names
external int _ICU4XBidiInfo_paragraph_count(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XBidiInfo_paragraph_at')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XBidiInfo_paragraph_at(ffi.Pointer<ffi.Opaque> self, int n);

@ffi.Native<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XBidiInfo_size')
// ignore: non_constant_identifier_names
external int _ICU4XBidiInfo_size(ffi.Pointer<ffi.Opaque> self);

@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XBidiInfo_level_at')
// ignore: non_constant_identifier_names
external int _ICU4XBidiInfo_level_at(ffi.Pointer<ffi.Opaque> self, int pos);
