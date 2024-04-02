import 'package:alic/imagefiles.dart';
import 'package:data_table_2/data_table_2.dart';
import 'package:flutter/material.dart';
import 'package:open_file_macos/open_file_macos.dart';
import 'package:signals/signals_flutter.dart';

final _openFileMacosPlugin = OpenFileMacos();

class FilesTable extends StatefulWidget {
  const FilesTable({super.key});

  @override
  State<FilesTable> createState() => _FilesTableState();
}

class _FilesTableState extends State<FilesTable> {
  int? _currentSortColumn;
  bool _isSortAsc = true;
  List<ImageFile> rows = [];

  @override
  void initState() {
    debugPrint('initState');
    super.initState();
    ImageFiles.signal.listen(context, () {
      setState(() {
        rows = [...ImageFiles.signal];
      });
    });
    setState(() {
      rows = [...ImageFiles.signal];
    });
  }

  Widget getStatusIcon(ImageFile file) {
    var theme = Theme.of(context);
    return switch (file.status) {
      ImageFileStatus.success => Tooltip(
          message: 'Success', child: Icon(Icons.check, color: theme.hintColor)),
      ImageFileStatus.error => Tooltip(
          message: file.errorMessage,
          child: Icon(Icons.error, color: theme.colorScheme.error),
        ),
      ImageFileStatus.unoptimized => Tooltip(
          message: file.status.value,
          child: Icon(Icons.remove_outlined, color: theme.hintColor),
        ),
      ImageFileStatus.pending => Tooltip(
          message: file.status.value,
          child: SizedBox(
            height: 20,
            width: 20,
            child: Icon(Icons.pending, color: theme.disabledColor),
          ),
        ),
      _ => Tooltip(
          message: file.status.value,
          child: SizedBox(
            height: 20,
            width: 20,
            child: Icon(Icons.compress, color: theme.hintColor),
          ),
        ),
    };
  }

  _createDataTable() {
    return DataTable2(
      isVerticalScrollBarVisible: true,
      headingRowDecoration: const BoxDecoration(
        border: Border(
          bottom: BorderSide(
            width: 1,
          ),
        ),
      ),
      headingRowHeight: 40,
      dataRowHeight: 35,
      dividerThickness: 0.2,
      columnSpacing: 12,
      horizontalMargin: 12,
      minWidth: 600,
      columns: _createColumns(),
      rows: _createRows(),
      sortColumnIndex: _currentSortColumn,
      sortAscending: _isSortAsc,
      showCheckboxColumn: false,
    );
  }

  List<DataRow2> _createRows() {
    return List<DataRow2>.generate(
        rows.length,
        (index) => DataRow2(onSelectChanged: (_) {}, cells: [
              DataCell(getStatusIcon(rows[index])),
              DataCell(
                Tooltip(
                  waitDuration: const Duration(milliseconds: 500),
                  message: rows[index].path,
                  child: Text(
                    rows[index].file,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                onDoubleTap: () {
                  _openFileMacosPlugin.open(rows[index].path,
                      viewInFinder: true);
                },
              ),
              DataCell(Text(rows[index].sizeHumanReadable)),
              DataCell(Text(rows[index].savings)),
            ]));
  }

  List<DataColumn2> _createColumns() {
    void sorter(
        Comparable Function(ImageFile) getter, columnIndex, bool ascending) {
      setState(() {
        _currentSortColumn = columnIndex;
        _isSortAsc = ascending;
        if (ascending) {
          rows.sort((a, b) => getter(b).compareTo(getter(a)));
        } else {
          rows.sort((a, b) => getter(a).compareTo(getter(b)));
        }
      });
    }

    var status = DataColumn2(
      fixedWidth: 32,
      label: const Text(''),
      onSort: (columnIndex, asc) {
        sorter((d) => d.status.value, columnIndex, asc);
      },
    );
    var file = DataColumn2(
      size: ColumnSize.L,
      label: const Text('File '),
      onSort: (columnIndex, asc) {
        sorter((d) => d.file, columnIndex, asc);
      },
    );
    var size = DataColumn2(
      fixedWidth: 100,
      label: const Text('Size '),
      onSort: (columnIndex, asc) {
        sorter((d) => d.tableSize, columnIndex, asc);
      },
    );
    var savings = DataColumn2(
      fixedWidth: 100,
      label: const Text('Savings '),
      onSort: (columnIndex, asc) {
        sorter((d) => d.savings, columnIndex, asc);
      },
    );
    return [status, file, size, savings];
  }

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context);
    if (rows.isEmpty) {
      return Center(
          child: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(20),
          color: theme.focusColor,
          border: Border.all(
              color: theme.primaryColor.withAlpha(40),
              width: 2,
              style: BorderStyle.solid),
        ),
        child: Icon(
          Icons.file_download,
          color: theme.iconTheme.color!.withAlpha(40),
          size: 200,
        ),
      ));
    }
    return _createDataTable();
  }
}
