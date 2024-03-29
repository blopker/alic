// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, unused_field, unused_import, public_member_api_docs, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

import 'package:dogs_core/dogs_core.dart';

Future initialiseDogs() async {
  var engine = DogEngine.hasValidInstance ? DogEngine.instance : DogEngine();
  engine.registerAllConverters([]);
  engine.setSingleton();
}
