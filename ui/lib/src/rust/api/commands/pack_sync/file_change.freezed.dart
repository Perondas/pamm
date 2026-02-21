// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'file_change.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ChangeType {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ChangeType);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ChangeType()';
}


}

/// @nodoc
class $ChangeTypeCopyWith<$Res>  {
$ChangeTypeCopyWith(ChangeType _, $Res Function(ChangeType) __);
}


/// Adds pattern-matching-related methods to [ChangeType].
extension ChangeTypePatterns on ChangeType {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ChangeType_Created value)?  created,TResult Function( ChangeType_Deleted value)?  deleted,TResult Function( ChangeType_Modified value)?  modified,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ChangeType_Created() when created != null:
return created(_that);case ChangeType_Deleted() when deleted != null:
return deleted(_that);case ChangeType_Modified() when modified != null:
return modified(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ChangeType_Created value)  created,required TResult Function( ChangeType_Deleted value)  deleted,required TResult Function( ChangeType_Modified value)  modified,}){
final _that = this;
switch (_that) {
case ChangeType_Created():
return created(_that);case ChangeType_Deleted():
return deleted(_that);case ChangeType_Modified():
return modified(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ChangeType_Created value)?  created,TResult? Function( ChangeType_Deleted value)?  deleted,TResult? Function( ChangeType_Modified value)?  modified,}){
final _that = this;
switch (_that) {
case ChangeType_Created() when created != null:
return created(_that);case ChangeType_Deleted() when deleted != null:
return deleted(_that);case ChangeType_Modified() when modified != null:
return modified(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt size)?  created,TResult Function()?  deleted,TResult Function( PlatformInt64 sizeChange,  BigInt dlSize)?  modified,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ChangeType_Created() when created != null:
return created(_that.size);case ChangeType_Deleted() when deleted != null:
return deleted();case ChangeType_Modified() when modified != null:
return modified(_that.sizeChange,_that.dlSize);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt size)  created,required TResult Function()  deleted,required TResult Function( PlatformInt64 sizeChange,  BigInt dlSize)  modified,}) {final _that = this;
switch (_that) {
case ChangeType_Created():
return created(_that.size);case ChangeType_Deleted():
return deleted();case ChangeType_Modified():
return modified(_that.sizeChange,_that.dlSize);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt size)?  created,TResult? Function()?  deleted,TResult? Function( PlatformInt64 sizeChange,  BigInt dlSize)?  modified,}) {final _that = this;
switch (_that) {
case ChangeType_Created() when created != null:
return created(_that.size);case ChangeType_Deleted() when deleted != null:
return deleted();case ChangeType_Modified() when modified != null:
return modified(_that.sizeChange,_that.dlSize);case _:
  return null;

}
}

}

/// @nodoc


class ChangeType_Created extends ChangeType {
  const ChangeType_Created({required this.size}): super._();
  

 final  BigInt size;

/// Create a copy of ChangeType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ChangeType_CreatedCopyWith<ChangeType_Created> get copyWith => _$ChangeType_CreatedCopyWithImpl<ChangeType_Created>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ChangeType_Created&&(identical(other.size, size) || other.size == size));
}


@override
int get hashCode => Object.hash(runtimeType,size);

@override
String toString() {
  return 'ChangeType.created(size: $size)';
}


}

/// @nodoc
abstract mixin class $ChangeType_CreatedCopyWith<$Res> implements $ChangeTypeCopyWith<$Res> {
  factory $ChangeType_CreatedCopyWith(ChangeType_Created value, $Res Function(ChangeType_Created) _then) = _$ChangeType_CreatedCopyWithImpl;
@useResult
$Res call({
 BigInt size
});




}
/// @nodoc
class _$ChangeType_CreatedCopyWithImpl<$Res>
    implements $ChangeType_CreatedCopyWith<$Res> {
  _$ChangeType_CreatedCopyWithImpl(this._self, this._then);

  final ChangeType_Created _self;
  final $Res Function(ChangeType_Created) _then;

/// Create a copy of ChangeType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? size = null,}) {
  return _then(ChangeType_Created(
size: null == size ? _self.size : size // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ChangeType_Deleted extends ChangeType {
  const ChangeType_Deleted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ChangeType_Deleted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ChangeType.deleted()';
}


}




/// @nodoc


class ChangeType_Modified extends ChangeType {
  const ChangeType_Modified({required this.sizeChange, required this.dlSize}): super._();
  

 final  PlatformInt64 sizeChange;
 final  BigInt dlSize;

/// Create a copy of ChangeType
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ChangeType_ModifiedCopyWith<ChangeType_Modified> get copyWith => _$ChangeType_ModifiedCopyWithImpl<ChangeType_Modified>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ChangeType_Modified&&(identical(other.sizeChange, sizeChange) || other.sizeChange == sizeChange)&&(identical(other.dlSize, dlSize) || other.dlSize == dlSize));
}


@override
int get hashCode => Object.hash(runtimeType,sizeChange,dlSize);

@override
String toString() {
  return 'ChangeType.modified(sizeChange: $sizeChange, dlSize: $dlSize)';
}


}

/// @nodoc
abstract mixin class $ChangeType_ModifiedCopyWith<$Res> implements $ChangeTypeCopyWith<$Res> {
  factory $ChangeType_ModifiedCopyWith(ChangeType_Modified value, $Res Function(ChangeType_Modified) _then) = _$ChangeType_ModifiedCopyWithImpl;
@useResult
$Res call({
 PlatformInt64 sizeChange, BigInt dlSize
});




}
/// @nodoc
class _$ChangeType_ModifiedCopyWithImpl<$Res>
    implements $ChangeType_ModifiedCopyWith<$Res> {
  _$ChangeType_ModifiedCopyWithImpl(this._self, this._then);

  final ChangeType_Modified _self;
  final $Res Function(ChangeType_Modified) _then;

/// Create a copy of ChangeType
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? sizeChange = null,Object? dlSize = null,}) {
  return _then(ChangeType_Modified(
sizeChange: null == sizeChange ? _self.sizeChange : sizeChange // ignore: cast_nullable_to_non_nullable
as PlatformInt64,dlSize: null == dlSize ? _self.dlSize : dlSize // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
