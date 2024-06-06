Widget buildRow() {
  return Container(
      child: Row(
        children: <Widget>[
          const MyLogo(),
          const Expanded(
            child: Text('...'),
          ),
        ],
      )
  );
}
