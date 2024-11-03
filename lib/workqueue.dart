// ignore_for_file: avoid_print

// A work queue that takes a function and arguments, then runs the function with the arguments in a stream.
// Also has a maximum number of concurrent workers that can be set.

import 'dart:async';
import 'dart:io';
import 'dart:math';
import 'package:alic/log.dart';

class _WorkQueue {
  int _maxWorkers = max(2, Platform.numberOfProcessors - 2);
  final _queue = <Future<void> Function()>[];
  var _workers = 0;

  _WorkQueue({int? maxWorkers}) {
    if (maxWorkers != null) {
      _maxWorkers = maxWorkers;
    }
  }

  void add(Future<void> Function() work) {
    _queue.add(work);
    _run();
  }

  void join() async {
    while (_queue.isNotEmpty || _workers > 0) {
      await Future.delayed(const Duration(milliseconds: 100));
    }
  }

  void _run() {
    if (_workers < _maxWorkers && _queue.isNotEmpty) {
      _workers++;
      _queue.removeAt(0)().whenComplete(() {
        _workers--;
        _run();
      });
    }
  }
}

final workQueue = _WorkQueue();

void main(List<String> args) {
  var queue = _WorkQueue(maxWorkers: 4);
  for (var i = 0; i < 10; i++) {
    queue.add(() async {
      log.d('Running $i');
      await Future.delayed(const Duration(seconds: 1));
      log.d('Done $i');
    });
  }
  queue.join();
}
