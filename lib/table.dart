import 'package:flutter/material.dart';

class FilesTable extends StatefulWidget {
  const FilesTable({super.key, required this.rows});
  final List<Map<String, dynamic>> rows;

  @override
  State<FilesTable> createState() => _FilesTableState();
}

class _FilesTableState extends State<FilesTable> {
  int _currentSortColumn = 0;
  int? _currentSelectedRow;
  bool _isSortAsc = true;
  List<String> columns = [
    'Status',
    'File',
    'Size',
    'Savings',
  ];
  List<Map<String, dynamic>> rows = [];

  @override
  void initState() {
    super.initState();
    rows = List.of(widget.rows);
  }

  DataTable _createDataTable() {
    return DataTable(
      columns: _createColumns(),
      rows: _createRows(),
      sortColumnIndex: _currentSortColumn,
      sortAscending: _isSortAsc,
      showCheckboxColumn: false,
    );
  }

  List<DataColumn> _createColumns() {
    return columns
        .map((column) => DataColumn(
              label: Row(
                children: [
                  Text(
                    column,
                  ),
                  const SizedBox(width: 5),
                ],
              ),
              onSort: (columnIndex, _) {
                setState(() {
                  _currentSortColumn = columnIndex;
                  var columnId = columns[columnIndex].toLowerCase();
                  if (_isSortAsc) {
                    rows.sort((a, b) => b[columnId].compareTo(a[columnId]));
                  } else {
                    rows.sort((a, b) => a[columnId].compareTo(b[columnId]));
                  }
                  _isSortAsc = !_isSortAsc;
                });
              },
            ))
        .toList();
  }

  List<DataRow> _createRows() {
    var dataRows = <DataRow>[];
    for (final (index, row) in rows.indexed) {
      dataRows.add(DataRow(
        color: index % 2 == 0
            ? MaterialStateProperty.resolveWith<Color>(
                (Set<MaterialState> states) {
                return Colors.black26;
              })
            : MaterialStateProperty.resolveWith<Color>(
                (Set<MaterialState> states) {
                return Colors.white10;
              }),
        cells: [
          DataCell(Text(
            '#${row['status']}',
          )),
          _createTitleCell(row['file']),
          DataCell(Text(
            row['size'] as String,
          )),
          DataCell(Text(
            row['savings'] as String,
          ))
        ],
        onSelectChanged: (value) {},
      ));
    }
    return dataRows;
  }

  DataCell _createTitleCell(bookTitle) {
    return DataCell(Text(
      bookTitle,
    ));
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Theme(
          data: Theme.of(context).copyWith(
              // change icon color
              iconTheme: const IconThemeData(color: Colors.white70),
              dataTableTheme: DataTableThemeData(
                dataRowColor: MaterialStateProperty.resolveWith<Color>(
                    (Set<MaterialState> states) {
                  if (states.contains(MaterialState.selected)) {
                    return Theme.of(context)
                        .colorScheme
                        .primary
                        .withOpacity(0.08);
                  }
                  return Colors.black12;
                }),
                headingRowColor: MaterialStateProperty.resolveWith<Color>(
                    (Set<MaterialState> states) {
                  return Colors.white10;
                }),
                // horizontalMargin: 0,
                // columnSpacing: 0,
                dividerThickness: 0,
                headingTextStyle: const TextStyle(
                    color: Colors.white70, fontWeight: FontWeight.bold),
                dataTextStyle: const TextStyle(color: Colors.white70),
                headingRowHeight: 40,
              )),
          child: _createDataTable()),
    );
  }
}
