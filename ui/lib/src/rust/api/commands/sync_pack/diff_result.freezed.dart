// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'diff_result.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AddonChange {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AddonChange);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AddonChange()';
}


}

/// @nodoc
class $AddonChangeCopyWith<$Res>  {
$AddonChangeCopyWith(AddonChange _, $Res Function(AddonChange) __);
}


/// Adds pattern-matching-related methods to [AddonChange].
extension AddonChangePatterns on AddonChange {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AddonChange_Created value)?  created,TResult Function( AddonChange_Deleted value)?  deleted,TResult Function( AddonChange_Modified value)?  modified,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AddonChange_Created() when created != null:
return created(_that);case AddonChange_Deleted() when deleted != null:
return deleted(_that);case AddonChange_Modified() when modified != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AddonChange_Created value)  created,required TResult Function( AddonChange_Deleted value)  deleted,required TResult Function( AddonChange_Modified value)  modified,}){
final _that = this;
switch (_that) {
case AddonChange_Created():
return created(_that);case AddonChange_Deleted():
return deleted(_that);case AddonChange_Modified():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AddonChange_Created value)?  created,TResult? Function( AddonChange_Deleted value)?  deleted,TResult? Function( AddonChange_Modified value)?  modified,}){
final _that = this;
switch (_that) {
case AddonChange_Created() when created != null:
return created(_that);case AddonChange_Deleted() when deleted != null:
return deleted(_that);case AddonChange_Modified() when modified != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt size)?  created,TResult Function()?  deleted,TResult Function( List<FileChange> changes)?  modified,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AddonChange_Created() when created != null:
return created(_that.size);case AddonChange_Deleted() when deleted != null:
return deleted();case AddonChange_Modified() when modified != null:
return modified(_that.changes);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt size)  created,required TResult Function()  deleted,required TResult Function( List<FileChange> changes)  modified,}) {final _that = this;
switch (_that) {
case AddonChange_Created():
return created(_that.size);case AddonChange_Deleted():
return deleted();case AddonChange_Modified():
return modified(_that.changes);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt size)?  created,TResult? Function()?  deleted,TResult? Function( List<FileChange> changes)?  modified,}) {final _that = this;
switch (_that) {
case AddonChange_Created() when created != null:
return created(_that.size);case AddonChange_Deleted() when deleted != null:
return deleted();case AddonChange_Modified() when modified != null:
return modified(_that.changes);case _:
  return null;

}
}

}

/// @nodoc


class AddonChange_Created extends AddonChange {
  const AddonChange_Created({required this.size}): super._();
  

 final  BigInt size;

/// Create a copy of AddonChange
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AddonChange_CreatedCopyWith<AddonChange_Created> get copyWith => _$AddonChange_CreatedCopyWithImpl<AddonChange_Created>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AddonChange_Created&&(identical(other.size, size) || other.size == size));
}


@override
int get hashCode => Object.hash(runtimeType,size);

@override
String toString() {
  return 'AddonChange.created(size: $size)';
}


}

/// @nodoc
abstract mixin class $AddonChange_CreatedCopyWith<$Res> implements $AddonChangeCopyWith<$Res> {
  factory $AddonChange_CreatedCopyWith(AddonChange_Created value, $Res Function(AddonChange_Created) _then) = _$AddonChange_CreatedCopyWithImpl;
@useResult
$Res call({
 BigInt size
});




}
/// @nodoc
class _$AddonChange_CreatedCopyWithImpl<$Res>
    implements $AddonChange_CreatedCopyWith<$Res> {
  _$AddonChange_CreatedCopyWithImpl(this._self, this._then);

  final AddonChange_Created _self;
  final $Res Function(AddonChange_Created) _then;

/// Create a copy of AddonChange
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? size = null,}) {
  return _then(AddonChange_Created(
size: null == size ? _self.size : size // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class AddonChange_Deleted extends AddonChange {
  const AddonChange_Deleted(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AddonChange_Deleted);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AddonChange.deleted()';
}


}




/// @nodoc


class AddonChange_Modified extends AddonChange {
  const AddonChange_Modified({required final  List<FileChange> changes}): _changes = changes,super._();
  

 final  List<FileChange> _changes;
 List<FileChange> get changes {
  if (_changes is EqualUnmodifiableListView) return _changes;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_changes);
}


/// Create a copy of AddonChange
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AddonChange_ModifiedCopyWith<AddonChange_Modified> get copyWith => _$AddonChange_ModifiedCopyWithImpl<AddonChange_Modified>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AddonChange_Modified&&const DeepCollectionEquality().equals(other._changes, _changes));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_changes));

@override
String toString() {
  return 'AddonChange.modified(changes: $changes)';
}


}

/// @nodoc
abstract mixin class $AddonChange_ModifiedCopyWith<$Res> implements $AddonChangeCopyWith<$Res> {
  factory $AddonChange_ModifiedCopyWith(AddonChange_Modified value, $Res Function(AddonChange_Modified) _then) = _$AddonChange_ModifiedCopyWithImpl;
@useResult
$Res call({
 List<FileChange> changes
});




}
/// @nodoc
class _$AddonChange_ModifiedCopyWithImpl<$Res>
    implements $AddonChange_ModifiedCopyWith<$Res> {
  _$AddonChange_ModifiedCopyWithImpl(this._self, this._then);

  final AddonChange_Modified _self;
  final $Res Function(AddonChange_Modified) _then;

/// Create a copy of AddonChange
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? changes = null,}) {
  return _then(AddonChange_Modified(
changes: null == changes ? _self._changes : changes // ignore: cast_nullable_to_non_nullable
as List<FileChange>,
  ));
}


}

// dart format on
