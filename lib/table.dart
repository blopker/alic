import 'package:flutter/material.dart';
import 'package:imageoptimflutter/imagefiles.dart';
import 'package:open_file_macos/open_file_macos.dart';
import 'package:signals/signals_flutter.dart';

class FilesTable extends StatefulWidget {
  const FilesTable({super.key});

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
  List<ImageFile> rows = [];

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
                    rows.sort((a, b) =>
                        b.toJson()[columnId].compareTo(a.toJson()[columnId]));
                  } else {
                    rows.sort((a, b) =>
                        a.toJson()[columnId].compareTo(b.toJson()[columnId]));
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
            row.status,
          )),
          DataCell(
            Row(
              children: [
                ConstrainedBox(
                  constraints: const BoxConstraints(
                      maxWidth: 400, minWidth: 0, minHeight: 20, maxHeight: 20),
                  child: Flexible(
                    child: Text(
                      row.file,
                      overflow: TextOverflow.fade,
                      softWrap: false,
                    ),
                  ),
                ),
                const Spacer(),
                IconButton(
                  iconSize: 20,
                  icon: const Icon(Icons.folder_open_outlined),
                  onPressed: () async {
                    final openFileMacosPlugin = OpenFileMacos();
                    await openFileMacosPlugin.open(row.path,
                        viewInFinder: true);
                  },
                ),
              ],
            ),
            onDoubleTap: () async {
              final openFileMacosPlugin = OpenFileMacos();
              await openFileMacosPlugin.open(row.path, viewInFinder: true);
            },
          ),
          DataCell(Text(
            row.sizeAfterOptimization == null
                ? row.size.toString()
                : row.sizeAfterOptimization.toString(),
          )),
          DataCell(Text(
            row.savings,
          ))
        ],
        onSelectChanged: (value) {},
      ));
    }
    return dataRows;
  }

  @override
  Widget build(BuildContext context) {
    ImageFiles.signal.listen(context, () {
      setState(() {
        rows = [...ImageFiles.signal];
      });
    });
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
