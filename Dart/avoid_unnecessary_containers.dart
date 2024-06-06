import 'package:flutter/material.dart';

Widget buildRow() {
  return Container(
      child: Row(
        children: <Widget>[
          const Expanded(
            child: Text('...'),
          ),
        ],
      )
  );
}
