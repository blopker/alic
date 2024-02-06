import 'package:data_table_2/data_table_2.dart';
import 'package:flutter/material.dart';
import 'package:imageoptimflutter/imagefiles.dart';
import 'package:signals/signals_flutter.dart';

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
    return switch (file.status) {
      ImageFileStatus.success => const Tooltip(
          message: 'Success', child: Icon(Icons.check, color: Colors.green)),
      ImageFileStatus.error => Tooltip(
          message: file.errorMessage,
          child: const Icon(Icons.error, color: Colors.red),
        ),
      ImageFileStatus.unoptimized => Tooltip(
          message: file.status.value,
          child: const Icon(Icons.remove_outlined, color: Colors.orange),
        ),
      ImageFileStatus.pending => Tooltip(
          message: file.status.value,
          child: const SizedBox(
            height: 20,
            width: 20,
            child: Icon(Icons.pending, color: Colors.white70),
          ),
        ),
      _ => Tooltip(
          message: file.status.value,
          child: const SizedBox(
            height: 20,
            width: 20,
            child: CircularProgressIndicator(
              color: Colors.orange,
              strokeWidth: 2,
            ),
          ),
        ),
    };
  }

  _createDataTable() {
    return DataTable2(
      headingRowHeight: 40,
      dataRowHeight: 35,
      dividerThickness: 1,
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

  List<DataRow> _createRows() {
    return List<DataRow>.generate(
        rows.length,
        (index) => DataRow(cells: [
              DataCell(getStatusIcon(rows[index])),
              DataCell(Text(rows[index].file)),
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
        sorter((d) => d.size, columnIndex, asc);
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
    if (rows.isEmpty) {
      return Center(
          child: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(20),
          color: Colors.black12,
          border: Border.all(
              color: Colors.white30, width: 2, style: BorderStyle.solid),
        ),
        child: const Icon(
          Icons.file_download,
          color: Colors.white30,
          size: 200,
        ),
      ));
    }
    return Theme(
        data: Theme.of(context).copyWith(
            iconTheme: const IconThemeData(color: Colors.white70),
            dataTableTheme: const DataTableThemeData(
              dataTextStyle: TextStyle(fontSize: 12, color: Colors.white70),
              headingTextStyle: TextStyle(fontSize: 14, color: Colors.white70),
              dividerThickness: 10,
            )),
        child: _createDataTable());
  }
}
